mod core;
mod frontend;
use crate::ast::ModuleAst;
use crate::error::CompileError;

pub struct TypeChecker<'a> {
    core: core::TypeCheckerCore,
    bindings: frontend::Bindings<'a>,
}

impl<'a> TypeChecker<'a> {
    pub fn new() -> Self {
        Self {
            core: core::TypeCheckerCore::new(),
            bindings: frontend::Bindings::new_module_scope(),
        }
    }

    pub fn check_module(&mut self, module: &ModuleAst) -> Result<(), CompileError> {
        frontend::check_module(&mut self.core, &mut self.bindings, module)
    }
}
