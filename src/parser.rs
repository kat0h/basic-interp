use crate::structs::*;

peg::parser! {
  pub grammar parse_line() for str {
    rule _() = [' ' | '\t']*

    rule number() -> i64
        = n:$(['0'..='9']+) {? n.parse::<i64>().or(Err("i64")) }

    rule name() -> String
        = n:$(['a'..='z']+) { n.to_string() }

    rule expr() -> Expr
        = precedence!{
            l:(@) _ "+" _ r:@ { Expr::BinOp { op: Op::Add, left: Box::new(l), right: Box::new(r), } }
            l:(@) _ "-" _ r:@ { Expr::BinOp { op: Op::Sub, left: Box::new(l), right: Box::new(r), } }
            --
            l:(@) _ "*" _ r:@ { Expr::BinOp { op: Op::Mul, left: Box::new(l), right: Box::new(r), } }
            l:(@) _ "/" _ r:@ { Expr::BinOp { op: Op::Div, left: Box::new(l), right: Box::new(r), } }
            --
            l:(@) _ "<" _ r:@ { Expr::BinOp { op: Op::LT, left: Box::new(l), right: Box::new(r), } }
            l:(@) _ ">" _ r:@ { Expr::BinOp { op: Op::GT, left: Box::new(l), right: Box::new(r), } }
            --
            n:number() { Expr::Num(n) }
            n:name() { Expr::Var(n) }
            "(" _ e:expr() _ ")" { e }
        }

    rule line() -> Stmt
        = n:number() s:$([^'\n']*) {
            Stmt::Line {
                index: n,
                line: s.to_string(),
            }
        }

    rule command() -> Stmt
        = s:$(['a'..='z']+) _ n:(expr() ** (" " _)) { Stmt::Command { command_name: s.to_string(), items: n } }

    rule assign() -> Stmt
        = n:name() _ "=" _ v:expr() { Stmt::Assign { name: n, value: v } }

    rule ifstmt() -> Stmt
        = "if" _ e:expr() _ "then" _ l:stmt() { Stmt::If { cond: e, iftrue: Box::new(l) } }

    rule stmt() -> Stmt
        = n:(line() / ifstmt() / assign() / command()) { n }

    pub rule input() -> Option<Stmt>
        = _ s:stmt()? _ "\n" { s }
  }
}

