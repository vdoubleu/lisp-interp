use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    Res,
    ASTNode
};
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;

pub fn interp_list(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    let mut lst = Vec::new();
    for child in children {
        let child_res = interp_ast(child, store, interp_args);
        lst.push(child_res);
    }
    lst.reverse();
    return Res::List(lst);
}