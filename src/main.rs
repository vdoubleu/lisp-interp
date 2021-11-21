mod runner;
mod interp;
mod ast_type;
mod tokenize;
mod ast_builder;

use std::io;
use crate::runner::run_prog;

pub fn main() {
    println!("Input Program:");

    let mut input_str: String = String::new();
    io::stdin().read_line(&mut input_str).expect("invalid read");

    let out = run_prog(&input_str);
    println!("Res: {}", out);
}

