mod interp_val;
mod interp_func;
mod interp_ast;
mod interp_let;
mod interp_if;
mod interp_nary;
mod interp_while;
mod utils;

use crate::ast_type::{
    ASTNode,
    Res,
};
use interp_ast::interp_ast;
use std::collections::HashMap;

pub fn interp(ast: &ASTNode) -> Res {
    let mut store: Vec<HashMap<String, Res>> = vec![HashMap::new()];
    //println!("{:#?}", ast);
    return interp_ast(&ast, &mut store);
}

