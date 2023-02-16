/*
 * Basic Interpriter
 */
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
                        println!("{}", v);
                    }
                }
                Stmt::Assign(val) => {
                    variable.insert(val.name.to_string(), val.value);
                }
            },
            Err(_) => {
                println!("Syntax Error!");
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
    items: Vec<i64>,
}

#[derive(Debug, PartialEq)]
pub struct StmtAssign {
    name: String,
    value: i64,
}

peg::parser! {
  grammar parse_line() for str {
    rule _() = [' ' | '\t']*

    rule number() -> i64
      = n:$(['0'..='9']+) {? n.parse::<i64>().or(Err("i64")) }

    rule print() -> Stmt
      = "print" _ n:number() {
          Stmt::Print(
              StmtPrint { items: vec![n] }
          )
      }

    rule assign() -> Stmt
      = n:$(['a'..='z']+) _ "=" _ v:number() {
          Stmt::Assign(
              StmtAssign { name: n.to_string(), value: v }
          )
      }

    pub rule line() -> Stmt
      = n:(print() / assign()) "\n" { n }
  }
}
