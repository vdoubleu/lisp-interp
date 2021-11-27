use crate::interp::interp;
use crate::ast_type::{
    ASTNode,
    Res
};
use crate::tokenize::tokenize;
use crate::ast_builder::build_ast;
use crate::cleaner::clean;


pub fn run_prog(prog: &String) -> String {
    let cleaned_prog: String = clean(&prog);
    let tokens: Vec<String> = tokenize(&cleaned_prog);
    let ast: Option<ASTNode> = build_ast(&tokens);

    match ast {
        Some(a) => {
            let res: Res = interp(&a);
            return res.to_string();
        },
        None    => panic!("No AST built"),
    }
}
