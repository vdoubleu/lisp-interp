use super::interp_ast::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res,
    NodeType
};
use crate::interp::utils::get_func_args_name_from_def;
use std::collections::HashMap;

pub fn interp_let(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if children.len() != 2 {
        panic!("Invalid number of children of let, expected 2, got: {}", children.len());
    }

    let let_var: &ASTNode = &(children[0]);
    match let_var.node_type {
        NodeType::Val => {
            let child_res: Res = interp_ast(&children[1], store);
            
            match store.last_mut() {
                Some(h) => h.insert(let_var.def.clone(), Res::clone(&child_res)),
                None    => panic!("Store has zero level in stack"),
            };
            return child_res;
        },
        NodeType::Function => {
            for c in &let_var.children {
                if !matches!(c.node_type, NodeType::Val) {
                    panic!("arg for function decl is not a variable: {:?}", c);
                }
            }

            let func_body = Res::Func(children[1].clone());

            let func_args = Res::Func(let_var.clone());
            let func_args_name = get_func_args_name_from_def(&let_var.def);

            match store.last_mut() {
                Some(h) => {
                    h.insert(let_var.def.clone(), Res::clone(&func_body));
                    h.insert(func_args_name, Res::clone(&func_args));
                },
                None    => panic!("Store has zero level in stack"),
            };

            return func_body;
        },
        _ => panic!("Cannot assign a value to this this var since it is not a func or a var"),
    }
}

