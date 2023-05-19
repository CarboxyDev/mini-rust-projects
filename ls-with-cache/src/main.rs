#![allow(dead_code)]
#![allow(unused_parens)]
#![allow(unused_must_use)]

use colored::Colorize;
use std::io::{stdout, Write};

fn shell_prompt() {
    let user = std::env::var("USER").expect("Error: Unable to find USER env var");
    let cwd = std::env::current_dir()
        .expect("Error: Unable to access CWD")
        .display()
        .to_string();
    let cwd_tokens: Vec<&str> = cwd.split("/").collect();
    let cwd_tail = cwd_tokens[cwd_tokens.len() - 1];

    print!("{} {} $ ", user.magenta(), cwd_tail.blue());
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

    let mut i: u32 = 0;
    for file in read_dir {
        i += 1;
        print!(
            "{}\t",
            file.unwrap()
                .path()
                .display()
                .to_string()
                .strip_prefix("./")
                .expect("Error: Failed to output file")
        );

        if (i % 3 == 0) {
            println!("");
        }
    }

    if (i % 3 != 0) {
        println!("");
    }
}

fn main() {
    print!("[!] Running Rust shell. \n\n");
    loop {
        shell_prompt();
        shell_input();
    }
}
