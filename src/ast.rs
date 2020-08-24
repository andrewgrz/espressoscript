#[derive(Debug, Clone)]
pub struct ModuleAst {
    pub statements: Vec<StatementAst>,
}

impl ModuleAst {
    pub fn from_statements(Statements: Vec<StatementAst>) -> ModuleAst {
        ModuleAst { statements }
    }
}

#[derive(Debug, Clone)]
pub enum StatementAst {
    Function(FunctionAst),
}

#[derive(Debug, Clone)]
pub struct FunctionAst {
    pub name: IdentAst,
    pub is_pub: bool,
    pub return_type: IdentAst,
    pub block: BlockAst,
}

#[derive(Debug, Clone)]
pub struct BlockAst {
    pub exprs: Vec<Box<ExprAst>>,
}

#[derive(Debug, Clone)]
pub struct ExprAst {
    pub exprs: Vec<Box<ExprAst>>,
}
