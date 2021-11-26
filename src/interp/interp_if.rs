use std::collections::HashMap;
use crate::interp::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res,
    DataType
};

pub fn interp_if(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        if children.len() >= 4 {
            panic!("Expected two or 3 args for if, but found: {}", children.len());
        }

        let r: Res = interp_ast(first, store);
        let cond: bool = match r.data_type {
            DataType::Bool(b)  => b,
            DataType::Int(i)   => i >= 1,
            DataType::NoRes    => false,
            _other  => panic!("Invalid return type function"),
        };

        if cond {
            return interp_ast(&rest[0], store);
        }

        if /* !cond && */ rest.len() == 2 {
            return interp_ast(&rest[1], store);
        }

        return Res::new();
    } else {
        panic!("Expected two or 3 args for if, but found: {}", children.len());
    }
}
