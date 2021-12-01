use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    ASTNode,
    Res,
};
use crate::reader::interp_args::InterpArgs;
use crate::interp::importer::import_module;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

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
    
    if paths_to_import.len() == 1 {
        return import_module(&paths_to_import[0], store, interp_args);
    } else {
        import_parallel(&paths_to_import, store, interp_args);
        return Res::NoRes;
    }
}

fn import_parallel(paths_to_import: &Vec<String>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    let (tx, rx) = mpsc::channel();
    let mut threads = Vec::new();

    for path in paths_to_import {
        let tx_clone = tx.clone();
        let path_clone = path.clone(); 
        let interp_args_copy = interp_args.clone();
        let t = thread::spawn(move || {
            let mut new_store: Vec<HashMap<String, Res>> = vec!(HashMap::new());
            import_module(&path_clone, &mut new_store, &interp_args_copy);
            tx_clone.send(new_store).unwrap();
        });

        threads.push(t);
    }
 
    drop(tx);


    for recieved in rx {
        if let Some(r) = recieved.last() {
            store.last_mut().unwrap().extend(r.into_iter().map(|(k, v)| (k.clone(), v.clone())));
        } else {
            panic!("import_parallel: recieved empty store");
        }
    }
    
    for t in threads {
        t.join().unwrap();
    }

    return Res::NoRes;
}