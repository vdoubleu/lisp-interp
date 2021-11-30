use crate::interp::interp_ast::interp_ast;
use crate::interp::interp_val::interp_val;
use crate::interp::utils::get_func_args_name_from_def;
use crate::ast_type::{
    ASTNode,
    Res
};
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;

pub fn interp_func(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    let func_body: Res = interp_val(&ast.def, store, interp_args,  true);

    match func_body {
        Res::Func(fb) => {
            let func_args_name = get_func_args_name_from_def(&ast.def);
            let func_args: Res = interp_val(&func_args_name, store, interp_args, true);

            match func_args {
                Res::Func(fa) => {
                    if fa.children.len() != ast.children.len() {
                        panic!("Incorrect number of arguments for function f. Given {}, expected {}", 
                               ast.children.len(), fa.children.len());
                    }

                    let func_arg_res: Vec<Res> = ast.children.iter().map(|c| interp_ast(c, store, interp_args)).collect();

                    // map func args
                    store.push(HashMap::new());
                    if let Some(func_store) = store.last_mut() {
                        for c_ind in 0..fa.children.len() {
                            let func_arg_def: String = fa.children[c_ind].def.clone();
                            let func_arg_val: Res = Res::clone(&func_arg_res[c_ind]);
                            func_store.insert(func_arg_def.clone(), Res::clone(&func_arg_val));
                        }
                    } else {
                        panic!("new level in store not properly added?");
                    }

                    let func_res: Res = interp_ast(&fb, store, interp_args);
                    store.pop();

                    return func_res;
                },
                _ => panic!("Expected function decl, but got something else: {}", &func_args_name),
            }
        },
        _ => panic!("Expected {} to be a function, but got something else", &ast.def),
    }
}
