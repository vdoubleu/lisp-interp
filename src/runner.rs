use crate::interp::interp;
use crate::ast_type::{
    ASTNode,
    Res
};
use crate::tokenize::tokenize;
use crate::ast_builder::build_ast;

pub fn run_prog(input_str: &String) -> String {
    let tokens: Vec<String> = tokenize(&input_str);
    let ast: Option<ASTNode> = build_ast(&tokens);

    match ast {
        Some(a) => {
            let res: Res = interp(&a);
            return res.to_string();
        },
        None    => panic!("No AST built"),
    }
}
