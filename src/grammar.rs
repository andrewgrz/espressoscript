use crate::ast::*;

peg::parser! {
  pub grammar grammar() for str {
    pub rule module() -> ModuleAst
      = s:statement()* { ModuleAst::from_statements(s) }

    rule statement() -> StatementAst
      = _ s:function_stmt() _ { s }

    rule function_stmt() -> StatementAst
      = s:function_def() { StatementAst::Function(s) }

    rule function_def() -> FunctionAst
      = is_pub:("pub")? _ "def" _ name:ident() _ "(" _ fn_args:arg() ** "," _ ","? _ ")" _ "->" _ r_type:ident() _ exprs:block() _
      { FunctionAst::new(name, is_pub.is_some(), fn_args, r_type, exprs) }

    rule arg() -> FunctionArgAst
      = _ name:ident() _ ":" _ arg_type:ident() _ { FunctionArgAst { name, arg_type } }

    rule block() -> BlockAst
      = "{" exprs:expr() ** ";" _ trailing_semi:";"? _ "}" { BlockAst::new(exprs, trailing_semi.is_some()) }

    rule ident() -> IdentAst
      = n:$(['a'..='z' | 'A'..='Z' | '_']['a'..='z' | 'A'..='Z' | '_']*) { IdentAst::from_str(n) }

    rule expr() -> ExprAst = precedence!{
       x:(@) "+" y:@ { ExprAst::Binary {lhs: Box::new(x), op: BinOpAst::Add, rhs: Box::new(y)} }
       x:(@) "-" y:@ { ExprAst::Binary {lhs: Box::new(x), op: BinOpAst::Sub, rhs: Box::new(y)} }
       --
       x:(@) "*" y:@ { ExprAst::Binary {lhs: Box::new(x), op: BinOpAst::Mul, rhs: Box::new(y)} }
       x:(@) "/" y:@ { ExprAst::Binary {lhs: Box::new(x), op: BinOpAst::Div, rhs: Box::new(y)} }
       --
       // For all rules in this level, we need to provide spaces around the expr
       _ n:(if_expr() / assignment() / function_call() / integer() / bool() / variable()) _ { n }
       _ "(" _ e:expr() _ ")" _ { e }
    }

    rule if_expr() -> ExprAst
      = "if" _ cond_expr:expr() _ then_expr:block() _ "else" _ else_expr:block() _
      { ExprAst::If { cond_expr: Box::new(cond_expr), then_expr, else_expr } }

    rule assignment() -> ExprAst
      = "let" _ name:ident() _ maybe_type:assign_type()? _"=" _ expr:expr() { ExprAst::Assignment { name, maybe_type, expr: Box::new(expr) } }

    rule assign_type() -> IdentAst
      = ":" _ i:ident() { i }

    rule function_call() -> ExprAst
      = name:ident() "(" exprs:expr() ** "," _ ","? ")" { ExprAst::new_fn_call(name, exprs) }

    rule variable() -> ExprAst
      = i:ident() { ExprAst::Variable(i) }

    rule bool() -> ExprAst
      = i:(bool_true() / bool_false()) { i }

    rule bool_true() -> ExprAst
      = i:"true" { ExprAst::Boolean(true) }

    rule bool_false() -> ExprAst
      = i:"false" { ExprAst::Boolean(false) }

    rule integer() -> ExprAst
      = n:$(['0'..='9']+) { ExprAst::Integer(n.parse().unwrap()) } / expected!("integer")

    rule _() = quiet!{[' ' | '\n' | '\t']*}
  }
}
