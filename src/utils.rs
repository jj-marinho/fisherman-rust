use std::io::{stdout, Write};

// Given a message, asks for user input
pub fn input(msg: &str) -> String {
    print!("{}", msg);
    stdout().flush().unwrap();
    let mut s = String::new();
    let _ = std::io::stdin().read_line(&mut s).unwrap();

    s.to_lowercase()
}
