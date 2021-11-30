use crate::interp::{
    interp_list::interp_list,
    interp_import::interp_import,
    interp_builtin::interp_builtin,
    interp_nary::interp_nary_op,
    interp_let::interp_let,
    interp_while::interp_while,
    interp_if::interp_if,
    interp_func::interp_func,
    interp_val::interp_val,
};
use crate::ast_type::{
    ASTNode,
    Res,
    NodeType,
};
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;

pub fn interp_ast(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    match ast.node_type {
        NodeType::NaryOp => {
            let interped_children: Vec<Res> = ast.children.iter().map(|c| interp_ast(&c, store, interp_args)).collect();
            return interp_nary_op(&ast.def, &interped_children);
        }
        NodeType::Val => interp_val(&ast.def, &store, false),
        NodeType::Let => interp_let(&ast.children, store, interp_args),
        NodeType::Seq => {
            let mut res: Res = Res::NoRes;
            for s in &ast.children {
                res = interp_ast(s, store, interp_args);
            }
            return res;
        },
        NodeType::While => interp_while(&ast.children, store, interp_args),
        NodeType::If => interp_if(&ast.children, store, interp_args), 
        NodeType::Import => interp_import(&ast.children, store, interp_args),
        NodeType::BuiltinFunction => interp_builtin(&ast.def, &ast.children, store, interp_args),
        NodeType::Function => interp_func(&ast, store, interp_args),
        NodeType::Empty => {
            panic!("Interping empty node type");
        },
        NodeType::List => interp_list(&ast.children, store, interp_args),
    }
}
