use crate::interp::interp_val::interp_val;
use crate::interp::interp_func::interp_func;
use crate::interp::interp_if::interp_if;
use crate::interp::interp_while::interp_while;
use crate::interp::interp_let::interp_let;
use crate::interp::interp_nary::interp_nary_op;
use crate::ast_type::{
    ASTNode,
    Res,
    NodeType,
};
use std::collections::HashMap;

pub fn interp_ast(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>) -> Res {
    match ast.node_type {
        NodeType::NaryOp => {
            let interped_children: Vec<Res> = ast.children.iter().map(|c| interp_ast(&c, store)).collect();
            return interp_nary_op(&ast.def, &interped_children);
        }
        NodeType::Val => interp_val(&ast.def, &store, false),
        NodeType::Let => interp_let(&ast.children, store),
        NodeType::Seq => {
            let mut res: Res = Res::new();
            for s in &ast.children {
                res = interp_ast(s, store);
            }
            return res;
        },
        NodeType::While => interp_while(&ast.children, store),
        NodeType::If => interp_if(&ast.children, store), 
        NodeType::Print => {
            let r: Res = interp_ast(&ast.children[0], store);
            println!("{}", r.to_string());
            return r;
        },
        NodeType::Function => interp_func(&ast, store),
        NodeType::Empty => {
            panic!("Interping empty node type");
        },
    }
}
