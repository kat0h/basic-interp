/*
 * Basic Interpriter
 */

fn main() {
    mainloop();
}

fn mainloop() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");

    println!("{}", line);
}
