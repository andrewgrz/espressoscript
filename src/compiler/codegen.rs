use crate::ast::{BlockAst, ExprAst, ModuleAst, StatementAst};
use crate::error::CompileError;
use std::fs::File;
use std::path::Path;

pub type WriteResult = Result<(), CompileError>;

fn write_file<P>(output_path: P, output: &String) -> WriteResult
where
    P: AsRef<Path>,
{
    use std::io::Write;
    let mut file = File::create(output_path).expect("Couldn't create output file");
    file.write_all(output.as_bytes())?;
    Ok(())
}

pub fn compile_to_file<P>(module: ModuleAst, output_path: P) -> WriteResult
where
    P: AsRef<Path>,
{
    let compiler = Compiler::new();
    let string = compiler.compile_module(module)?;
    write_file(output_path, &string)?;
    Ok(())
}

struct Compiler;

impl Compiler {
    fn new() -> Compiler {
        Compiler {}
    }
    fn compile_module(&self, module_ast: ModuleAst) -> Result<String, CompileError> {
        use std::fmt::Write;

        let mut output: String = String::new();
        for stmt in module_ast.statements {
            match stmt {
                StatementAst::Function(function) => {
                    write!(output, "function {}(", function.name)?;
                    // Add Params
                    for arg in &function.fn_args {
                        write!(output, "{},", arg.name)?;
                    }
                    write!(output, "){{")?;
                    self.visit_block(&function.block, &mut output)?;
                    write!(output, "}}")?;
                    writeln!(output)?;
                }
            }
        }
        write!(
            output,
            "document.getElementById(\"app-root\").innerHTML = main(true)"
        )?;
        Ok(output)
    }

    fn visit_block(&self, block: &BlockAst, output: &mut String) -> Result<(), CompileError> {
        use std::fmt::Write;
        let length = block.exprs.len();

        for (index, expr) in block.exprs.iter().enumerate() {
            if index + 1 == length && !block.trailing_semi {
                write!(output, "return ")?;
            }
            self.visit_expr(expr, output)?;
            write!(output, ";")?;
        }

        Ok(())
    }

    fn visit_expr(&self, expr: &Box<ExprAst>, output: &mut String) -> Result<(), CompileError> {
        use std::fmt::Write;

        match **expr {
            ExprAst::Integer(ref i) => write!(output, "{}", i)?,
            ExprAst::Boolean(ref f) => write!(output, "{}", f)?,
            ExprAst::Variable(ref v) => write!(output, "{}", v)?,
            ExprAst::FunctionCall { ref name, ref args } => {
                write!(output, "{}(", name)?;
                for arg in args {
                    self.visit_expr(&arg, output)?;
                    write!(output, ",")?;
                }
                write!(output, ")")?;
            }
            ExprAst::Assignment {
                ref name, ref expr, ..
            } => {
                write!(output, "const {}=", name)?;
                self.visit_expr(&expr, output)?;
            }
            ExprAst::Binary {
                ref lhs,
                ref op,
                ref rhs,
            } => {
                self.visit_expr(&lhs, output)?;
                write!(output, "{}", op)?;
                self.visit_expr(&rhs, output)?;
            }
            ExprAst::If {
                ref cond_expr,
                ref then_expr,
                ref else_expr,
            } => {
                write!(output, "(function(){{if(")?;
                self.visit_expr(cond_expr, output)?;
                write!(output, "){{")?;
                self.visit_block(then_expr, output)?;
                write!(output, "}}else{{")?;
                self.visit_block(else_expr, output)?;
                write!(output, "}}}})()")?;
            }
        }
        Ok(())
    }
}
