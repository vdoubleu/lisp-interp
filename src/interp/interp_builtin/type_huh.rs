use crate::reader::interp_args::InterpArgs;
use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    Res,
    ASTNode
};
use std::collections::HashMap;


pub fn type_huh(def: &String, args: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, target_type: Res, interp_args: &InterpArgs) -> Res {
    if args.len() == 0 {
        panic!("{} needs atleast one argument", def);
    }

    let mut out = true;
    for arg in args {
        if std::mem::discriminant(&target_type) == std::mem::discriminant(&interp_ast(arg, store, interp_args)) {
            // do nothing
        } else {
            out = false;
        }
    }
    return Res::Bool(out);    
}
