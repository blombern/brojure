use std::io::{stdin,stdout,Write};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

mod node;
mod parser;
mod eval;

use parser::{tokenize, parse};
use eval::{Env, eval};

fn main() {
    let mut env: Env = HashMap::new();
    let mut initialized = false;

    loop {
        if !initialized {
            let std = read_file("lib/lib.clj");
            eval(&mut parse(&mut tokenize(std.as_ref())), &mut env);
            initialized = true;
        }
        let mut input = String::new();
        print!("λ> ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("");

        if let Some('\n') = input.chars().next_back() {
            input.pop();
        }
        if let Some('\r') = input.chars().next_back() {
            input.pop();
        }

        if input == ":exit" {
            println!("{}", "Bye!");
            std::process::exit(0);
        } else if input == ":load" {
            input = read_file("lib/lib.clj");
        }

        let mut tokens = tokenize(input.as_ref());
        let mut parsed = parse(&mut tokens);

        println!("{}", eval(&mut parsed, &mut env))
    }
}

fn read_file(filename: &str) -> String {
    let mut f = File::open(filename).expect("File not found");
    let mut s = String::new();
    f.read_to_string(&mut s)
        .expect("Something went wrong reading the file");
    s
}
