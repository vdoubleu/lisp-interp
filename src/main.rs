mod runner;
mod interp;
mod ast_type;
mod tokenize;
mod ast_builder;
mod reader;
mod cleaner;
mod checker;

use crate::runner::run_prog;
use crate::reader::{
    read,
    get_args
};
use crate::reader::interp_args::InterpArgs;

pub fn main() {
    let interp_args: InterpArgs = get_args();
    let prog: String = read(&interp_args);

    run_prog(&prog, &interp_args);
}

