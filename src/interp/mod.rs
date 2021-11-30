mod interp_val;
mod interp_func;
mod interp_ast;
mod interp_let;
mod interp_if;
mod interp_nary;
mod interp_while;
mod interp_builtin;
mod interp_import;
mod interp_list;
mod utils;

use crate::{ast_type::{
    ASTNode,
    Res,
}, reader::interp_args::InterpArgs};
use interp_ast::interp_ast;
use std::collections::HashMap;

pub fn interp(ast: &ASTNode, interp_args: &InterpArgs) -> Res {
    let mut store: Vec<HashMap<String, Res>> = vec![HashMap::new()];
    //println!("{:#?}", ast);
    return interp_ast(&ast, &mut store, &interp_args);
}

pub fn interp_with_store(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    return interp_ast(&ast, store, &interp_args);
}