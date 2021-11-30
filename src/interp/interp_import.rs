use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res,
};
use crate::reader::interp_args::InterpArgs;
use crate::interp::importer::import_module;
use std::collections::HashMap;

pub fn interp_import(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    if children.len() == 0 {
        panic!("import statement must have at least one child");
    }

    let paths_to_import: Vec<String> = children.iter().map(|x| {
        let r = interp_ast(x, store, interp_args);
        match r {
            Res::Str(s) => s.clone(),
            _ => panic!("import: expected string {}", r.to_string()),
        }
    }).collect();
    
    let mut import_res: Res = Res::NoRes;
    for path in paths_to_import {
        import_res = import_module(&path, store, interp_args);
    }

    return import_res;
}

