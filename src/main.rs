/*
 * Basic Interpriter
 */
use std::io;

fn main() {
    mainloop();
}

fn mainloop() {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");

    println!("{}", line);
}

peg::parser! {
  grammar parse_line() for str {
    rule print() -> STMTPrint
      = "print" n:$['0'..='9'] { STMTPrint(n.parse().or(Err("print arg error")) }

    pub rule line() -> STMT
      = n:print() { STMT::Print(n) }
  }
}

#[derive(Debug, PartialEq)]
enum STMT {
    Print(STMTPrint)
}

#[derive(Debug, PartialEq)]
struct STMTPrint {
    items: Vec<i64>,
}
