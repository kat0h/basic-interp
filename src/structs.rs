use std::collections::HashMap;

pub enum ReturnCode {
    Ok_,
    SilentOk_,
    Error_,
}

// 環境
pub struct Env {
    // 変数 - 名前: 数値の対応
    pub variable: HashMap<String, i64>,
    // プログラム - 行番号: 内容
    pub line: HashMap<i64, String>,
    // 内蔵コマンド - 名前: 構造体
    pub command: HashMap<String, InternalCommand>,
    // プログラムカウンタ - 次実行するべき行
    //   -1のときは次に実行する行がない
    pub nl: i64,
}

pub struct InternalCommand {
    pub func: fn(env: &mut Env, args: &[Expr]) -> ReturnCode,
    pub argl: i64,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub index: Option<i64>,
    pub stmt: Stmt,
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
    pub fn exec(&self, env: &mut Env) -> ReturnCode {
        // エラー時の処理をきちんと作成する
        match self {
            // 行番号
            Stmt::Line { index, line } => {
                // TODO: env.nlを書き換える
                env.line.insert(*index, line.to_string());
                ReturnCode::SilentOk_
            }
            // if文
            Stmt::If { cond, iftrue } => match cond.eval(env) {
                Some(v) => {
                    if v != 0 {
                        iftrue.exec(env);
                    }
                    ReturnCode::Ok_
                }
                None => {
                    // println!("Undefined Variable");
                    ReturnCode::Error_
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
                        // eprintln!("Too many/less argumants");
                        return ReturnCode::Error_;
                    }
                    (cmd.func)(env, items)
                } else {
                    // eprintln!("Undefined Command");
                    ReturnCode::Error_
                }
            }
            // 変数の割り当て
            Stmt::Assign { name, value } => match value.eval(env) {
                Some(v) => {
                    env.variable.insert(name.to_string(), v);
                    ReturnCode::Ok_
                }
                None => {
                    // eprintln!("Undefined Variable");
                    ReturnCode::Error_
                }
            },
        }
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


