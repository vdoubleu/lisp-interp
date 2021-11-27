use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res,
};
use crate::runner::run_prog_with_store;
use crate::reader::read;
use std::collections::HashMap;

pub fn interp_import(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if children.len() == 0 {
        panic!("import statement must have at least one child");
    }

    let paths_to_import: Vec<String> = children.iter().map(|x| {
        let r = interp_ast(x, store);
        match r {
            Res::Str(s) => s.clone(),
            _ => panic!("import: expected string {}", r.to_string()),
        }
    }).collect();
    
    let mut import_res: Res = Res::NoRes;
    for path in paths_to_import {
        import_res = import_module(&path, store);
    }

    return import_res;
}

fn import_module(path: &String, store: &mut Vec<HashMap<String, Res>>) -> Res {
    let file_args: Vec<String> = vec!("_".to_string(), path.clone());
    let file_contents = read(&file_args);
    
    let res = run_prog_with_store(&file_contents, store);
    return res;
}