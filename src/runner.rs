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
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;


pub fn run_prog(prog: &String, interp_args: &InterpArgs) -> Res {
    let ast: Option<ASTNode> = prep_prog(prog, &interp_args);

    if interp_args.no_interp{
        return Res::NoRes;
    }

    match ast {
        Some(a) => {
            let res: Res = interp(&a, &interp_args);
            return res;
        },
        None    => panic!("No AST built"),
    }
}

pub fn run_prog_with_store(prog: &String, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    let ast: Option<ASTNode> = prep_prog(prog, &interp_args);

    if interp_args.no_interp{
        return Res::NoRes;
    }

    match ast {
        Some(a) => {
            let res: Res = interp_with_store(&a, store, &interp_args);
            return res;
        },
        None    => panic!("No AST built"),
    }
}

fn prep_prog(prog: &String, interp_args: &InterpArgs) -> Option<ASTNode> {
    let cleaned_prog: String = clean(&prog);
    let tokens: Vec<String> = tokenize(&cleaned_prog);

    if interp_args.print_tokens {
        println!("Tokens: {:?}", tokens);
    }

    let ast: Option<ASTNode> = build_ast(&tokens);

    return ast;
}
