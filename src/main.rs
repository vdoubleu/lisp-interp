mod runner;
mod interp;
mod ast_type;
mod tokenize;
mod ast_builder;
mod reader;
mod cleaner;

use std::env;
use crate::runner::run_prog;
use crate::reader::read;

pub fn main() {
    let input_args: Vec<String> = env::args().collect();

    let prog: String = read(&input_args);

    run_prog(&prog);
}

