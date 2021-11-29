use std::collections::HashMap;
use crate::interp::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res
};
use crate::reader::interp_args::InterpArgs;

pub fn interp_if(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    if let Some((first, rest)) = children.split_first() {
        if children.len() >= 4 {
            panic!("Expected two or 3 args for if, but found: {}", children.len());
        }

        let r: Res = interp_ast(first, store, interp_args);
        let cond: bool = match r {
            Res::Bool(b)  => b,
            Res::Int(i)   => i >= 1,
            Res::NoRes    => false,
            _other  => panic!("Invalid return type function"),
        };

        if cond {
            return interp_ast(&rest[0], store, interp_args);
        }

        if /* !cond && */ rest.len() == 2 {
            return interp_ast(&rest[1], store, interp_args);
        }

        return Res::NoRes;
    } else {
        panic!("Expected two or 3 args for if, but found: {}", children.len());
    }
}
