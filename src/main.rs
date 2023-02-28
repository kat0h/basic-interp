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

mod parser;
mod structs;
use crate::parser::parse_line;
use crate::structs::*;

fn main() {
    mainloop();
}

fn mainloop() {
    // TODO 初期化関数を作る
    let mut env = Env {
        variable: HashMap::new(),
        line: HashMap::new(),
        command: HashMap::new(),
        nl: -1,
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
        },
    );

    loop {
        // dbg!(&env.variable);
        // dbg!(&env.line);
        // dbg!(&env.nl);
        // println!();
 
        // 処理する行を取得
        let mut line = String::new();
        // 処理はREPLの入力か
        let is_in_repl;
        if env.nl == -1 {
            // 行をREPLから読み込み
            let bytes = io::stdin()
                .read_line(&mut line)
                .expect("Failed to read line");
            if bytes == 0 {
                break;
            }
            is_in_repl = true;
        } else if let Some(l) = env.line.get(&env.nl) {
            line = l.to_string() + "\n";
            // 次の行を取得
            env.nl = {
                let mut indexlist = env.line.iter().map(|i| *(i.0)).collect::<Vec<i64>>();
                indexlist.sort();
                match indexlist.binary_search(&env.nl) {
                    Ok(n) => *indexlist.get(n+1).unwrap_or(&-1),
                    Err(_) => -1,
                }
            };
            is_in_repl = false;
        } else {
            dbg!("Error!: Can't find line");
            continue;
        }
        let is_last_line = !is_in_repl && (env.nl == -1);

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
                            // REPLか最終行にいる
                            if is_in_repl || is_last_line {
                                println!("OK");
                            }
                        }
                        ReturnCode::SilentOk_ => (),
                        ReturnCode::Error_ => {
                            println!("Error");
                            // エラーが起きたら実行を中断する
                            env.nl = -1;
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

// 保存されている全ての行を表示
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
    let mut indexlist = env.line.iter().map(|i| *(i.0)).collect::<Vec<i64>>();
    indexlist.sort();
    // 実行中にrunコマンドが実行された場合、無限ループする
    if let Some(first_line) = indexlist.first() {
        env.nl = *first_line;
        ReturnCode::Ok_
    } else {
        env.nl = -1;
        ReturnCode::Error_
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
