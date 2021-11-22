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
use crate::cleaner::clean;

pub fn main() {
    let input_args: Vec<String> = env::args().collect();

    let prog: String = read(&input_args);
    let cleaned_prog: String = clean(&prog);

    run_prog(&cleaned_prog);
}

