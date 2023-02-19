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
    let mut env = Env {
        variable: HashMap::new(),
        line: HashMap::new(),
        command: HashMap::new(),
    };

    // 組み込みコマンドを初期化する
    env.command.insert(
        "print".to_string(),
        InternalCommand {
            func: command_print,
            argl: -1,
        },
    );
    env.command.insert(
        "list".to_string(),
        InternalCommand {
            func: command_list,
            argl: 0,
        },
    );
    env.command.insert(
        "run".to_string(),
        InternalCommand {
            func: command_run,
            argl: 0,
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
            Ok(stmt) => {
                if let Some(stmt) = stmt {
                    stmt.exec(&mut env);
                }
            }
            // パーサーがエラーを吐いた場合
            Err(err) => {
                println!("Syntax Error!");
                dbg!(&err);
            }
        }
    }
}

fn command_print(env: &mut Env, args: &[Expr]) {
    match args.iter().map(|v| v.eval(env)).collect::<Option<Vec<_>>>() {
        Some(n) => {
            println!(
                "{}",
                n.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
        None => {
            eprintln!("Evaluation Error!");
        }
    }
}

fn command_list(env: &mut Env, _: &[Expr]) {
    let mut l: Vec<(&i64, &String)> = env.line.iter().collect();
    l.sort_by(|a, b| a.0.cmp(b.0));
    for i in l.iter() {
        println!("{}{}", i.0, i.1);
    }
}

fn command_run(env: &mut Env, _: &[Expr]) {
    // TODO: 実行中に新しい行が追加された時の対応
    let mut indexlist = env.line.iter().map(|i| *(i.0)).collect::<Vec<i64>>();
    indexlist.sort();
    for index in indexlist {
        let parsed = parse_line::input(&(env.line.get(&index).unwrap().to_string() + "\n"));
        match parsed {
            Ok(stmt) => {
                if let Some(stmt) = stmt {
                    stmt.exec(env);
                }
            }
            // パーサーがエラーを吐いた場合
            Err(err) => {
                println!("Syntax Error!");
                dbg!(&err);
            }
        }
    }
}

// 環境
pub struct Env {
    variable: HashMap<String, i64>,
    line: HashMap<i64, String>,
    command: HashMap<String, InternalCommand>,
}

type Icommand = fn(env: &mut Env, args: &[Expr]);
pub struct InternalCommand {
    func: Icommand,
    argl: i64,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    index: Option<i64>,
    stmt: Stmt,
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    // 行の登録
    Line {
        index: i64,
        line: String,
    },
    // 代入文を表す
    Assign {
        name: String,
        value: Expr,
    },
    // if文を表す
    If {
        cond: Expr,
        iftrue: Box<Stmt>,
    },
    // print等のコマンドを表す
    Command {
        command_name: String,
        items: Vec<Expr>,
    },
}

impl Stmt {
    pub fn exec(&self, env: &mut Env) {
        // エラー時の処理をきちんと作成する
        match self {
            // 行番号
            Stmt::Line { index, line } => {
                env.line.insert(*index, line.to_string());
            }
            // if文
            Stmt::If { cond, iftrue } => match cond.eval(env) {
                Some(v) => {
                    if v != 0 {
                        iftrue.exec(env);
                    }
                }
                None => {
                    println!("Undefined Variable");
                }
            },
            // コマンド呼び出し
            Stmt::Command {
                command_name,
                items,
            } => {
                // コマンドの存在確認
                if let Some(cmd) = env.command.get(command_name) {
                    // 引数の数をチェック
                    if cmd.argl != -1 && cmd.argl as usize != items.len() {
                        eprintln!("Too many/less argumants");
                        return;
                    }
                    (cmd.func)(env, items);
                } else {
                    eprintln!("Undefined Command");
                }
            }
            // 変数の割り当て
            Stmt::Assign { name, value } => match value.eval(env) {
                Some(v) => {
                    env.variable.insert(name.to_string(), v);
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
    pub fn eval(&self, env: &mut Env) -> Option<i64> {
        match self {
            Expr::Num(n) => Some(*n),
            Expr::Var(n) => env.variable.get(n).copied(),
            Expr::BinOp { op, left, right } => {
                let l = left.eval(env)?;
                let r = right.eval(env)?;
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
