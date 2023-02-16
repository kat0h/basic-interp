/*
 * Basic Interpriter
 */
use std::io;

fn main() {
    mainloop();
}

fn mainloop() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        //parse line
        let parsed = parse_line::line(&line);
        match parsed {
            Ok(stmt) => {
                match stmt {
                    Stmt::Print(val) => {
                        for v in val.items.iter() {
                            println!("{}", v);
                        }
                    }
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
}

#[derive(Debug, PartialEq)]
pub struct StmtPrint {
    items: Vec<i64>,
}

peg::parser! {
  grammar parse_line() for str {
    rule _() = [' ' | '\t']*

    rule number() -> i64
      = n:$(['0'..='9']+) {? n.parse::<i64>().or(Err("i64")) }

    rule print() -> StmtPrint
      = "print" _ n:number() { StmtPrint{ items: vec![n] }}

    pub rule line() -> Stmt
      = n:print() "\n" { Stmt::Print(n) }
  }
}
