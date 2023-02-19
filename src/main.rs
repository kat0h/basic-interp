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
    let mut lines: HashMap<i64, Box<Stmt>> = HashMap::new();
    let mut variable: HashMap<String, i64> = HashMap::new();
    let mut command: HashMap<String, InternalCommand> = HashMap::new();

    // 組み込みコマンドを初期化する
    command.insert(
        "print".to_string(),
        InternalCommand {
            func: command_print,
            argl: 1,
        },
    );

    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        //parse line
        let parsed = parse_line::input(&line);
        match parsed {
            Ok(line) => match line.index {
                Some(i) => {
                    lines.insert(i, Box::new(line.stmt));
                }
                None => {
                    line.stmt.exec(&mut variable, &mut lines, &mut command);
                }
            },
            Err(err) => {
                println!("Syntax Error!");
                dbg!(&err);
            }
        }
    }
}

fn command_print(
    variable: &mut HashMap<String, i64>,
    _: &mut HashMap<i64, Box<Stmt>>,
    args: &[Expr],
) {
    for v in args.iter() {
        match v.eval(variable) {
            Some(n) => println!("{}", n),
            None => eprintln!("Evaluation Error!"),
        }
    }
}

type Icommand =
    fn(variable: &mut HashMap<String, i64>, lines: &mut HashMap<i64, Box<Stmt>>, args: &[Expr]);
pub struct InternalCommand {
    func: Icommand,
    argl: usize,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    index: Option<i64>,
    stmt: Stmt,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Command {
        command_name: String,
        items: Vec<Expr>,
    },
    Assign {
        name: String,
        value: Expr,
    },
    If {
        cond: Expr,
        iftrue: Box<Stmt>,
    },
}

impl Stmt {
    pub fn exec(
        &self,
        variable: &mut HashMap<String, i64>,
        lines: &mut HashMap<i64, Box<Stmt>>,
        command: &mut HashMap<String, InternalCommand>,
    ) {
        // エラー時の処理をきちんと作成する
        match self {
            Stmt::If { cond, iftrue } => match cond.eval(variable) {
                Some(v) => {
                    if v != 0 {
                        iftrue.exec(variable, lines, command);
                    }
                }
                None => {
                    println!("Undefined Variable");
                }
            },
            Stmt::Command {
                command_name,
                items,
            } => {
                // コマンドの存在確認
                if let Some(cmd) = command.get(command_name) {
                    if cmd.argl != items.len() {
                        eprintln!("Too many/less argumants");
                        return;
                    }
                    (cmd.func)(variable, lines, items);
                } else {
                    eprintln!("Undefined Command");
                }
            }
            Stmt::Assign { name, value } => match value.eval(variable) {
                Some(v) => {
                    variable.insert(name.to_string(), v);
                }
                None => {
                    eprintln!("Undefined Variable");
                }
            },
        };
    }
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
                    Op::LT => {
                        if l < r {
                            1
                        } else {
                            0
                        }
                    }
                    Op::GT => {
                        if l > r {
                            1
                        } else {
                            0
                        }
                    }
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

    rule command() -> Stmt
        = s:$(['a'..='z']+) _ n:expr() { Stmt::Command { command_name: s.to_string(), items: vec![n] } }

    rule assign() -> Stmt
        = n:name() _ "=" _ v:expr() { Stmt::Assign { name: n, value: v } }

    rule ifstmt() -> Stmt
        = "if" _ e:expr() _ "then" _ l:stmt() { Stmt::If { cond: e, iftrue: Box::new(l) } }

    rule stmt() -> Stmt
        = n:(command() / assign() / ifstmt()) { n }

    pub rule input() -> Line
        = i:(number()?) _ n:stmt() _ "\n" {
            Line {
                index: i,
                stmt: n
            }
        }
  }
}
