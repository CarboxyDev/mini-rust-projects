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

    let input = input
        .strip_suffix("\n")
        .expect("Error: Failed to parse the input");

    if (input == "ls") {
        ls();
    }
}

fn ls() {
    let read_dir = std::fs::read_dir("./").expect("Unable to read the current working directory");
    for file in read_dir {
        print!("{}\n", file.unwrap().path().display());
    }
}

fn main() {
    print!("[!] Running Rust shell. \n\n");
    loop {
        shell_prompt();
        shell_input();
    }
}
