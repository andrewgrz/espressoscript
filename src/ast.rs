use std::fmt;

/// Represented a Parsed Module
#[derive(Debug, Clone)]
pub struct ModuleAst {
    pub statements: Vec<StatementAst>,
}

impl ModuleAst {
    pub fn from_statements(statements: Vec<StatementAst>) -> ModuleAst {
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
    pub fn_args: Vec<FunctionArgAst>,
    pub return_type: IdentAst,
    pub block: BlockAst,
}

impl FunctionAst {
    pub fn new(
        name: IdentAst,
        is_pub: bool,
        fn_args: Vec<FunctionArgAst>,
        return_type: IdentAst,
        block: BlockAst,
    ) -> FunctionAst {
        FunctionAst {
            name,
            is_pub,
            fn_args,
            return_type,
            block,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockAst {
    pub exprs: Vec<Box<ExprAst>>,
    pub trailing_semi: bool,
}

impl BlockAst {
    pub fn new(exprs: Vec<ExprAst>, trailing_semi: bool) -> BlockAst {
        BlockAst {
            exprs: exprs.iter().map(|e| Box::new(e.clone())).collect(),
            trailing_semi,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionArgAst {
    pub name: IdentAst,
    pub arg_type: IdentAst,
}

#[derive(Debug, Clone)]
pub enum ExprAst {
    Boolean(bool),
    Integer(i64),
    Variable(IdentAst),
    FunctionCall {
        name: IdentAst,
        args: Vec<Box<ExprAst>>,
    },
    Assignment {
        name: IdentAst,
        maybe_type: Option<IdentAst>,
        expr: Box<ExprAst>,
    },
    Binary {
        lhs: Box<ExprAst>,
        op: BinOpAst,
        rhs: Box<ExprAst>,
    },
    If {
        cond_expr: Box<ExprAst>,
        then_expr: BlockAst,
        else_expr: BlockAst,
    },
}

impl ExprAst {
    pub fn new_fn_call(name: IdentAst, args: Vec<ExprAst>) -> ExprAst {
        ExprAst::FunctionCall {
            name,
            args: args.iter().map(|e| Box::new(e.clone())).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BinOpAst {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for BinOpAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinOpAst::Add => write!(f, "+"),
            BinOpAst::Sub => write!(f, "-"),
            BinOpAst::Mul => write!(f, "*"),
            BinOpAst::Div => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdentAst {
    pub name: String,
}
impl IdentAst {
    pub fn from_str(s: &str) -> IdentAst {
        IdentAst {
            name: s.to_string(),
        }
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn as_string(&self) -> String {
        self.name.clone()
    }
}

impl fmt::Display for IdentAst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
