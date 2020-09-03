use crate::ast::{ExprAst, IdentAst};
use crate::error::CompileError;
use crate::typechecker::core::{TypeCheckerCore, Value};
use std::collections::HashMap;

struct Bindings {
    m: HashMap<IdentAst, Value>,
}

impl Bindings {
    fn new() -> Self {
        Self { m: HashMap::new() }
    }

    fn get(&self, k: &IdentAst) -> Option<Value> {
        self.m.get(k).copied()
    }

    fn insert(&mut self, k: IdentAst, v: Value) {
        self.m.insert(k.clone(), v);
    }

    fn in_child_scope<T>(&mut self, cb: impl FnOnce(&mut Self) -> T) -> T {
        let mut child_scope = Bindings { m: self.m.clone() };
        cb(&mut child_scope)
    }
}

fn check_expr(
    engine: &mut TypeCheckerCore,
    bindings: &mut Bindings,
    expr: &Box<ExprAst>,
) -> Result<Value, CompileError> {
    use ExprAst::*;
    match **expr {
        Boolean(_) => Ok(engine.bool()),
        Integer(_) => unimplemented!(),
        Binary { .. } => unimplemented!(),
        Variable(ref name) => bindings.get(name).ok_or_else(|| {
            CompileError::SyntaxError(format!("Undefined variable {}", name)).into()
        }),
        FunctionCall { ref name, ref args } => {
            let func_type = bindings
                .get(name)
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
            bindings.in_child_scope(|bindings| {
                bindings.insert(name.clone(), var_type);
            });
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

            let then_type = check_expr(engine, bindings, then_expr)?;
            let else_type = check_expr(engine, bindings, else_expr)?;

            let (merged, merged_bound) = engine.var();
            engine.flow(then_type, merged_bound)?;
            engine.flow(else_type, merged_bound)?;
            Ok(merged)
        }
    }
}
