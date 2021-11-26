use crate::ast_type::{
    ASTNode,
    Res,
    DataType
};
use crate::interp::interp_ast::interp_ast;
use std::collections::HashMap;

pub fn interp_while(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        let mut r: Res = interp_ast(first, store);
        let mut keep_looping: bool = match r.data_type {
            DataType::Bool(b)  => b,
            DataType::Int(i)   => i >= 1,
            DataType::NoRes    => false,
            _other  => panic!("Invalid loop condition type"),
        };

        let mut res: Res = Res::new();
        while keep_looping {
            for s in rest {
                res = interp_ast(s, store);
            }

            r = interp_ast(first, store);
            keep_looping = match r.data_type {
                DataType::Bool(b)  => b,
                DataType::Int(i)   => i >= 1,
                DataType::NoRes    => false,
                _other  => panic!("Invalid return type function"),
            };
        }

        return res;
    } else {
        panic!("Expected atleast two args for while, but only found 0");
    }
}


