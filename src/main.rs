/*
 * Basic Interpriter
 */
use core::option::Option;
use std::collections::HashMap;
use std::io;

fn main() {
    mainloop();
}

fn mainloop() {
    let mut variable = HashMap::new();
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        //parse line
        let parsed = parse_line::line(&line);
        match parsed {
            Ok(stmt) => match stmt {
                Stmt::Print(val) => {
                    for v in val.items.iter() {
                        match v.eval(&variable) {
                            Some(n) => println!("{}", n),
                            None => eprintln!("Undefined Variable"),
                        }
                    }
                }
                Stmt::Assign(val) => match val.value.eval(&variable) {
                    Some(v) => {
                        variable.insert(val.name.to_string(), v);
                    }
                    None => {
                        eprintln!("Undefined Variable");
                    }
                },
            },
            Err(_) => {
                eprintln!("Syntax Error!");
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Print(StmtPrint),
    Assign(StmtAssign),
}

#[derive(Debug, PartialEq)]
pub struct StmtPrint {
    items: Vec<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct StmtAssign {
    name: String,
    value: Expr,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Num(i64),
    Var(String),
    BinOp {
        op: Op,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Op {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    LT,  // <
    GT,  // >
}

impl Expr {
    pub fn eval(&self, variable: &HashMap<String, i64>) -> Option<i64> {
        match self {
            Expr::Num(n) => Some(*n),
            Expr::Var(n) => variable.get(n).copied(),
            Expr::BinOp { op, left, right } => {
                let l = left.eval(variable)?;
                let r = right.eval(variable)?;
                Some(match op {
                    Op::Add => l + r,
                    Op::Sub => l - r,
                    Op::Mul => l * r,
                    Op::Div => l / r,
                    Op::LT  => if l < r { 1 } else { 0 },
                    Op::GT  => if l > r { 1 } else { 0 },
                })
            }
        }
    }
}

peg::parser! {
  grammar parse_line() for str {
    rule _() = [' ' | '\t']*

    rule number() -> i64
        = n:$(['0'..='9']+) {? n.parse::<i64>().or(Err("i64")) }

    rule name() -> String
        = n:$(['a'..='z']+) { n.to_string() }

    rule expr() -> Expr
        = precedence!{
            l:(@) _ "+" _ r:@ {
                Expr::BinOp {
                    op: Op::Add,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            l:(@) _ "-" _ r:@ {
                Expr::BinOp {
                    op: Op::Sub,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            --
            l:(@) _ "*" _ r:@ {
                Expr::BinOp {
                    op: Op::Mul,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            l:(@) _ "/" _ r:@ {
                Expr::BinOp {
                    op: Op::Div,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            --
            l:(@) _ "<" _ r:@ {
                Expr::BinOp {
                    op: Op::LT,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            l:(@) _ ">" _ r:@ {
                Expr::BinOp {
                    op: Op::GT,
                    left: Box::new(l),
                    right: Box::new(r),
                }
            }
            --
            n:number() { Expr::Num(n) }
            n:name() { Expr::Var(n) }
            "(" _ e:expr() _ ")" { e }
        }

    rule print() -> Stmt
        = "print" _ n:expr() {
            Stmt::Print(
                StmtPrint { items: vec![n] }
            )
        }

    rule assign() -> Stmt
        = n:name() _ "=" _ v:expr() {
            Stmt::Assign(
                StmtAssign { name: n, value: v }
            )
        }

    pub rule line() -> Stmt
        = n:(print() / assign()) "\n" { n }
  }
}
