use crate::ast_type::{
    ASTNode,
    NodeType,
    Res,
    DataType
};
use std::collections::HashMap;

pub fn interp(ast: &ASTNode) -> Res {
    let mut store: Vec<HashMap<String, Res>> = vec![HashMap::new()];
    //println!("{:#?}", ast);
    return interp_ast(&ast, &mut store);
}

fn interp_ast(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>) -> Res {
    match ast.node_type {
        NodeType::NaryOp => {
            let interped_children: Vec<Res> = ast.children.iter().map(|c| interp_ast(&c, store)).collect();
            return interp_nary_op(&ast.def, &interped_children);
        }
        NodeType::Val => interp_val(&ast.def, &store, false),
        NodeType::Let => interp_let(&ast.children, store),
        NodeType::Seq => {
            let mut res: Res = Res::new();
            for s in &ast.children {
                res = interp_ast(s, store);
            }
            return res;
        },
        NodeType::While => interp_while(&ast.children, store),
        NodeType::If => interp_if(&ast.children, store), 
        NodeType::Print => {
            let r: Res = interp_ast(&ast.children[0], store);
            println!("{}", r.to_string());
            return r;
        },
        NodeType::Function => interp_func(&ast, store),
        NodeType::Empty => {
            panic!("Interping empty node type");
        },
    }
}

fn get_func_args_name_from_def(def: &str) -> String {
    return format!("_{}", def);
}

fn interp_func(ast: &ASTNode, store: &mut Vec<HashMap<String, Res>>) -> Res {
    let func_body: Res = interp_val(&ast.def, store, true);

    match func_body.data_type {
        DataType::Func(fb) => {
            let func_args_name = get_func_args_name_from_def(&ast.def);
            let func_args: Res = interp_val(&func_args_name, store, true);

            match func_args.data_type {
                DataType::Func(fa) => {
                    if fa.children.len() != ast.children.len() {
                        panic!("Incorrect number of arguments for function f. Given {}, expected {}", 
                               ast.children.len(), fa.children.len());
                    }

                    let func_arg_res: Vec<Res> = ast.children.iter().map(|c| interp_ast(c, store)).collect();

                    // map func args
                    store.push(HashMap::new());
                    if let Some(func_store) = store.last_mut() {
                        for c_ind in 0..fa.children.len() {
                            let func_arg_def: String = fa.children[c_ind].def.clone();
                            let func_arg_val: Res = Res::copy(&func_arg_res[c_ind]);
                            func_store.insert(func_arg_def.clone(), Res::copy(&func_arg_val));
                        }
                    } else {
                        panic!("new level in store not properly added?");
                    }

                    let func_res: Res = interp_ast(&fb, store);
                    store.pop();

                    return func_res;
                },
                _ => panic!("Expected function decl, but got something else: {}", &func_args_name),
            }
        },
        _ => panic!("Expected {} to be a function, but got something else", &ast.def),
    }
}

fn interp_let(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if children.len() != 2 {
        panic!("Invalid number of children of let, expected 2, got: {}", children.len());
    }

    let let_var: &ASTNode = &(children[0]);
    match let_var.node_type {
        NodeType::Val => {
            let child_res: Res = interp_ast(&children[1], store);
            
            match store.last_mut() {
                Some(h) => h.insert(let_var.def.clone(), Res::copy(&child_res)),
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

            let func_body = Res::new_f(&children[1]);

            let func_args = Res::new_f(&let_var);
            let func_args_name = get_func_args_name_from_def(&let_var.def);

            match store.last_mut() {
                Some(h) => {
                    h.insert(let_var.def.clone(), Res::copy(&func_body));
                    h.insert(func_args_name, Res::copy(&func_args));
                },
                None    => panic!("Store has zero level in stack"),
            };

            return func_body;
        },
        _ => panic!("Cannot assign a value to this this var since it is not a func or a var"),
    }
}


fn interp_while(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        let mut r: Res = interp_ast(first, store);
        let mut keep_looping: bool = match r.data_type {
            DataType::Bool(b)  => b,
            DataType::Int(i)   => i >= 1,
            DataType::NoRes    => false,
            DataType::Func(_)  => panic!("Invalid return type function"),
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
                DataType::Func(_)  => panic!("Invalid return type function"),
            };
        }

        return res;
    } else {
        panic!("Expected atleast two args for while, but only found 0");
    }
}

fn interp_if(children: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        if children.len() >= 4 {
            panic!("Expected two or 3 args for if, but found: {}", children.len());
        }

        let r: Res = interp_ast(first, store);
        let cond: bool = match r.data_type {
            DataType::Bool(b)  => b,
            DataType::Int(i)   => i >= 1,
            DataType::NoRes    => false,
            DataType::Func(_)  => panic!("Invalid return type function"),
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

fn interp_val(def: &String, store: &Vec<HashMap<String, Res>>, permit_func: bool) -> Res {
    match def.parse::<i64>() {
        Ok(n) => return Res::new_i(n),
        Err(_) => {
            for s in store.iter().rev() {
                if s.contains_key(def) {
                    let r: Res = Res::copy(&s[def]);
                    match r.data_type {
                        DataType::NoRes => panic!("Found var with no val: {}", &def),
                        DataType::Bool(b)  => return Res::new_b(b),
                        DataType::Int(i)  => return Res::new_i(i),
                        DataType::Func(fb) => return if permit_func {
                                Res::new_f(&fb)
                            } else {
                                panic!("first class functions not supported");
                            },
                    }
                } 
            }

            return Res::new();
        }
    }
}

fn interp_nary_op(def: &String, args: &Vec<Res>) -> Res {
    for a in args {
        if !matches!(a.data_type, DataType::Int(_)) {
            panic!("Expected int type in nary op, got: {}", a.to_string());
        }
    }

    match def.as_ref() {
        "+" | "-" | "*" | "/"   => return Res::new_i(int_nary_op(&def, &args)),
        ">" | ">=" | "<" | "<=" | "==" => return Res::new_b(bool_nary_op(&def, &args)),
        _                       => panic!("Unrecognised nary op: {}", &def),
    }
}

fn int_nary_op(def: &String, args: &Vec<Res>) -> i64 {
    let int_args: Vec<i64> = args.iter().map(|x| {
        match x.data_type {
            DataType::Int(i) => i,
            _ => panic!("expected ints in nary op"),
        }
    }).collect();

    match def.as_ref() {
        "+" => int_args.iter().sum::<i64>(),
        "-" => {
            if let Some((first, rest)) = int_args.split_first() {
                return first - rest.iter().sum::<i64>();
            } else {
                panic!("Missing args for subtraction, given 0, expected more");
            }
        },
        "*" => int_args.iter().product::<i64>(),
        "/" => {
            if let Some((first, rest)) = int_args.split_first() {
                return first / rest.iter().product::<i64>();
            } else {
                panic!("Missing args for division, given 0, expected more");
            }
        },
        other => panic!("Unrecognised operation: {}", other),
    }
}

fn bool_nary_op(def: &String, args: &Vec<Res>) -> bool {
    if let Some((first, rest)) = args.split_first() {
        match def.as_ref() {
            ">"   => return rest.iter().all(|x| first.get_i() > x.get_i()),
            "<"   => return rest.iter().all(|x| first.get_i() < x.get_i()),
            "<="  => return rest.iter().all(|x| first.get_i() <= x.get_i()),
            ">="  => return rest.iter().all(|x| first.get_i() >= x.get_i()),
            "=="  => return rest.iter().all(|x| first.get_i() == x.get_i()),
            other =>  panic!("Unrecognised operation: {}", other),
        }
    } else {
        panic!("Missing args for comparison {}, expected more than 2, 0", &def);
    }
}

