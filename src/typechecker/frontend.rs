use crate::ast::{BlockAst, ExprAst, IdentAst, ModuleAst, StatementAst};
use crate::error::CompileError;
use crate::typechecker::core::{TypeCheckerCore, Value};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Bindings<'a> {
    parent: Option<&'a Bindings<'a>>,
    symbols: HashMap<IdentAst, Value>,
}

impl<'a> Bindings<'a> {
    pub fn new_module_scope() -> Bindings<'a> {
        let table = Bindings {
            parent: None,
            symbols: HashMap::new(),
        };
        // TODO: Add builtins to here
        table
    }

    fn push(&self) -> Bindings {
        Bindings {
            parent: Some(&self),
            symbols: HashMap::new(),
        }
    }

    fn insert(&mut self, k: IdentAst, v: Value) {
        self.symbols.insert(k, v);
    }

    fn resolve(&self, name: &IdentAst) -> Option<Value> {
        match self.symbols.get(name) {
            Some(s) => Some(*s),
            None => match &self.parent {
                Some(p) => p.resolve(name),
                None => None,
            },
        }
    }
}

#[must_use = "Result of Frontend checking must be used"]
pub fn check_module(
    engine: &mut TypeCheckerCore,
    bindings: &mut Bindings,
    module_ast: &ModuleAst,
) -> Result<(), CompileError> {
    for stmt in &module_ast.statements {
        match stmt {
            StatementAst::Function(func) => {
                let mut local_bindings = bindings.push();
                let mut arg_bounds = Vec::new();
                for arg in &func.fn_args {
                    let (arg_type, arg_bound) = engine.var();
                    local_bindings.insert(arg.name.clone(), arg_type);
                    arg_bounds.push(arg_bound);
                }
                let body_type = check_block(engine, &mut local_bindings, &func.block)?;
                engine.func(arg_bounds, body_type);
            }
        }
    }

    Ok(())
}

#[must_use = "Result of Frontend checking must be used"]
fn check_block(
    engine: &mut TypeCheckerCore,
    bindings: &mut Bindings,
    block: &BlockAst,
) -> Result<Value, CompileError> {
    if block.exprs.len() == 0 {
        return Ok(Value);
    }

    // Check all the expression in the block
    for x in 0..block.exprs.len() - 1 {
        check_expr(engine, bindings, &block.exprs.last().unwrap())?;
    }

    if block.trailing_semi {
        Ok(Value)
    } else {
        // This unwrap won't fail as we know the length is at least one
        check_expr(engine, bindings, &block.exprs.last().unwrap())
    }
}

#[must_use = "Result of Frontend checking must be used"]
fn check_expr(
    engine: &mut TypeCheckerCore,
    bindings: &mut Bindings,
    expr: &Box<ExprAst>,
) -> Result<Value, CompileError> {
    use ExprAst::*;
    match **expr {
        Boolean(_) => Ok(engine.bool()),
        Integer(_) => Ok(engine.integer()),
        Binary { .. } => unimplemented!(),
        Variable(ref name) => bindings.resolve(name).ok_or_else(|| {
            CompileError::SyntaxError(format!("Undefined variable {}", name)).into()
        }),
        FunctionCall { ref name, ref args } => {
            let func_type = bindings
                .resolve(name)
                .ok_or_else(|| CompileError::SyntaxError(format!("Undefined function {}", name)))?;

            let mut arg_types = Vec::new();
            for arg_expr in args {
                arg_types.push(check_expr(engine, bindings, arg_expr)?);
            }

            let (ret_type, ret_bound) = engine.var();
            let bound = engine.func_use(arg_types, ret_bound);
            engine.flow(func_type, bound)?;
            Ok(ret_type)
        }
        Assignment {
            ref name, ref expr, ..
        } => {
            let var_type = check_expr(engine, bindings, expr)?;
            bindings.insert(name.clone(), var_type);
            Ok(Value)
        }
        If {
            ref cond_expr,
            ref then_expr,
            ref else_expr,
        } => {
            let cond_type = check_expr(engine, bindings, cond_expr)?;
            let bound = engine.bool_use();
            engine.flow(cond_type, bound)?;

            let then_type = check_block(engine, &mut bindings.push(), then_expr)?;
            let else_type = check_block(engine, &mut bindings.push(), else_expr)?;

            let (merged, merged_bound) = engine.var();
            engine.flow(then_type, merged_bound)?;
            engine.flow(else_type, merged_bound)?;
            Ok(merged)
        }
    }
}
