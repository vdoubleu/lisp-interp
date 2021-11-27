use crate::interp::{
    interp,
    interp_with_store
};
use crate::ast_type::{
    ASTNode,
    Res
};
use crate::tokenize::tokenize;
use crate::ast_builder::build_ast;
use crate::cleaner::clean;
use std::collections::HashMap;


pub fn run_prog(prog: &String) -> Res {
    let ast: Option<ASTNode> = prep_prog(prog);

    match ast {
        Some(a) => {
            let res: Res = interp(&a);
            return res;
        },
        None    => panic!("No AST built"),
    }
}

pub fn run_prog_with_store(prog: &String, store: &mut Vec<HashMap<String, Res>>) -> Res {
    let ast: Option<ASTNode> = prep_prog(prog);

    match ast {
        Some(a) => {
            let res: Res = interp_with_store(&a, store);
            return res;
        },
        None    => panic!("No AST built"),
    }
}

fn prep_prog(prog: &String) -> Option<ASTNode> {
    let cleaned_prog: String = clean(&prog);
    let tokens: Vec<String> = tokenize(&cleaned_prog);
    let ast: Option<ASTNode> = build_ast(&tokens);

    return ast;
}