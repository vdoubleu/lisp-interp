use crate::ast_type::{
    ASTNode,
    Res
};
use crate::interp::interp_ast::interp_ast;
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;

pub fn interp_while(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    if let Some((first, rest)) = children.split_first() {
        let mut r: Res = interp_ast(first, store, interp_args);
        let mut keep_looping: bool = match r {
            Res::Bool(b)  => b,
            Res::Int(i)   => i >= 1,
            Res::NoRes    => false,
            _other  => panic!("Invalid loop condition type"),
        };

        let mut res: Res = Res::NoRes;
        while keep_looping {
            for s in rest {
                res = interp_ast(s, store, interp_args);
            }

            r = interp_ast(first, store, interp_args);
            keep_looping = match r {
                Res::Bool(b)  => b,
                Res::Int(i)   => i >= 1,
                Res::NoRes    => false,
                _other  => panic!("Invalid return type function"),
            };
        }

        return res;
    } else {
        panic!("Expected atleast two args for while, but only found 0");
    }
}


