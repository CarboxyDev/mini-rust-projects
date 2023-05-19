#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

use std::io::{stdout, Write};

fn shell_prompt() {
    let user = std::env::var("USER").expect("Unable to find USER env var");
    print!("> {} $ ", user);
}

fn shell_input() {
    let mut input = String::new();
    let _ = stdout().flush();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: Unable to take input from user");
}

fn main() {
    print!("[!] Running Rust shell. \n\n");
    loop {
        shell_prompt();
        shell_input();
    }
}
