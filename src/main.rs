/*
 * Basic Interpriter
 */

/*
 * 仕様
 *   文法
 *    - if [expr] then [stmt]
 *    - [name] = [expr]
 *   組み込みコマンド
 *    - print
 *    - cls
 *    - list
 *    - run
 *    - clear
 *   TODO
 *    - new
 *    - goto
 *    - 文字列型
 * 
 */

use core::option::Option;
use std::collections::HashMap;
use std::io;
use std::io::{stdout, Write};

mod structs;
mod parser;
use crate::structs::*;
use crate::parser::parse_line;


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
            func: icommand_print,
            argl: -1,
        },
    );
    env.command.insert(
        "list".to_string(),
        InternalCommand {
            func: icommand_list,
            argl: 0,
        },
    );
    env.command.insert(
        "run".to_string(),
        InternalCommand {
            func: icommand_run,
            argl: 0,
        },
    );
    env.command.insert(
        "cls".to_string(),
        InternalCommand {
            func: icommand_cls,
            argl: 0,
        },
    );
    env.command.insert(
        "clear".to_string(),
        InternalCommand {
            func: icommand_clear,
            argl: 0,
        }
    );

    loop {
        let mut line = String::new();
        let bytes = io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        //parse line
        let parsed = parse_line::input(&line);
        match parsed {
            // 正常にパースできた場合
            Ok(stmt) => {
                if let Some(stmt) = stmt {
                    let err = stmt.exec(&mut env);
                    // エラーを振り分け
                    match err {
                        ReturnCode::Ok_ => {
                            println!("OK");
                        }
                        ReturnCode::SilentOk_ => (),
                        ReturnCode::Error_ => {
                            println!("Error")
                        }
                    }
                }
            }
            // パーサーがエラーを吐いた場合
            Err(err) => {
                println!("Syntax Error!");
                dbg!(&line);
                dbg!(&err);
            }
        }
    }
}

// 画面に文字列を表示する
fn icommand_print(env: &mut Env, args: &[Expr]) -> ReturnCode {
    match args.iter().map(|v| v.eval(env)).collect::<Option<Vec<_>>>() {
        Some(n) => {
            println!(
                "{}",
                n.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            ReturnCode::Ok_
        }
        None => {
            eprintln!("Evaluation Error!");
            ReturnCode::Error_
        }
    }
}

// 保存されている全ての行を削除
fn icommand_list(env: &mut Env, _: &[Expr]) -> ReturnCode { 
    let mut l: Vec<(&i64, &String)> = env.line.iter().collect();
    l.sort_by(|a, b| a.0.cmp(b.0));
    for i in l.iter() {
        println!("{}{}", i.0, i.1);
    }
    ReturnCode::Ok_
}

// プログラムを実行
fn icommand_run(env: &mut Env, _: &[Expr]) -> ReturnCode {
    // TODO: 実行中に新しい行が追加された時の対応
    // 実行前に全ての行を取得して、順に実行する
    let mut indexlist = env.line.iter().map(|i| *(i.0)).collect::<Vec<i64>>();
    let mut rc: ReturnCode = ReturnCode::Ok_;
    indexlist.sort();
    for index in indexlist {
        let parsed = parse_line::input(&(env.line.get(&index).unwrap().to_string() + "\n"));
        rc = match parsed {
            Ok(stmt) => {
                if let Some(stmt) = stmt {
                    stmt.exec(env)
                } else {
                    // 空行など
                    ReturnCode::SilentOk_
                }
            }
            // パーサーがエラーを吐いた場合
            Err(err) => {
                // TODO
                println!("Syntax Error!");
                dbg!(&err);
                ReturnCode::Error_
            }
        };
        if let ReturnCode::Error_ = rc {
            break;
        }
    }
    if let ReturnCode::Error_ = rc {
        ReturnCode::Error_
    } else {
        ReturnCode::Ok_
    }
}

// 画面をクリア
fn icommand_cls(_: &mut Env, _: &[Expr]) -> ReturnCode {
    print!("\x1b[2J\x1b[H");
    stdout().flush().unwrap();
    ReturnCode::Ok_
}

// 全ての変数を削除
fn icommand_clear(env: &mut Env, _: &[Expr]) -> ReturnCode {
    env.variable = HashMap::new();
    ReturnCode::Ok_
}
