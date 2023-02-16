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
                        match v.get_num(&variable) {
                            Some(n) => println!("{}", n),
                            None => eprintln!("Undefined Variable"),
                        }
                    }
                }
                Stmt::Assign(val) => match val.value.get_num(&variable) {
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
    items: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub struct StmtAssign {
    name: String,
    value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Num(i64),
    Var(String),
}

impl Value {
    pub fn get_num(&self, variable: &HashMap<String, i64>) -> Option<i64> {
        match self {
            Value::Num(n) => Some(*n),
            Value::Var(n) => variable.get(n).copied(),
        }
    }
}

peg::parser! {
  grammar parse_line() for str {
    rule _() = [' ' | '\t']*

    rule number() -> i64
      = n:$(['0'..='9']+) {? n.parse::<i64>().or(Err("i64")) }

    rule value() -> Value
      = n:number() { Value::Num(n) }

    rule print() -> Stmt
      = "print" _ n:value() {
          Stmt::Print(
              StmtPrint { items: vec![n] }
          )
      }

    rule assign() -> Stmt
      = n:$(['a'..='z']+) _ "=" _ v:value() {
          Stmt::Assign(
              StmtAssign { name: n.to_string(), value: v }
          )
      }

    pub rule line() -> Stmt
      = n:(print() / assign()) "\n" { n }
  }
}
