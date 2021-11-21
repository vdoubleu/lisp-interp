use crate::ast_type::{
    ASTNode,
    NodeType,
    Res,
    DataType
};
use std::collections::HashMap;


pub fn interp(ast: &ASTNode) -> Res {
    let mut store: HashMap<String, Res> = HashMap::new();

    return interp_ast(&ast, &mut store);
}

fn interp_ast(ast: &ASTNode, store: &mut HashMap<String, Res>) -> Res {
    match ast.node_type {
        NodeType::NaryOp => {
            let interped_children: Vec<Res> = ast.children.iter().map(|c| interp_ast(&c, store)).collect();
            return interp_nary_op(&ast.def, &interped_children);
        }
        NodeType::Val => interp_val(&ast.def, &store),
        NodeType::Let => {
            assert!(ast.children.len() == 2);
            let child_res: Res = interp_ast(&ast.children[1], store);
            store.insert(ast.children[0].def.clone(), Res::copy(&child_res));
            return child_res;
        },
        NodeType::Seq => {
            let mut res: Res = Res::new();
            for s in &ast.children {
                res = interp_ast(s, store);
            }
            return res;
        },
        NodeType::While => interp_while(&ast.children, store),
        NodeType::If => interp_if(&ast.children, store), 
        NodeType::Empty => {
            panic!("Interping empty node type");
        },
    }
}

fn interp_while(children: &Vec<ASTNode>, store: &mut HashMap<String, Res>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        let mut r: Res = interp_ast(first, store);
        let mut keep_looping: bool = match r.data_type {
            DataType::Bool  => r.b_val,
            DataType::Int   => r.i_val >= 1,
            DataType::NoRes => false,
        };

        let mut res: Res = Res::new();
        while keep_looping {
            for s in rest {
                res = interp_ast(s, store);
            }

            r = interp_ast(first, store);
            keep_looping = match r.data_type {
                DataType::Bool  => r.b_val,
                DataType::Int   => r.i_val >= 1,
                DataType::NoRes => false,
            };
        }

        return res;
    } else {
        panic!("Expected atleast two args for while, but only found 0");
    }
}

fn interp_if(children: &Vec<ASTNode>, store: &mut HashMap<String, Res>) -> Res {
    if let Some((first, rest)) = children.split_first() {
        if children.len() >= 4 {
            panic!("Expected two or 3 args for if, but found: {}", children.len());
        }

        let r: Res = interp_ast(first, store);
        let cond: bool = match r.data_type {
            DataType::Bool  => r.b_val,
            DataType::Int   => r.i_val >= 1,
            DataType::NoRes => false,
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

fn interp_val(def: &String, store: &HashMap<String, Res>) -> Res {
    match def.parse::<i64>() {
        Ok(n) => return Res::new_i(n),
        Err(err) => {
            if store.contains_key(def) {
                let r: Res = Res::copy(&store[def]);
                match r.data_type {
                    DataType::NoRes => panic!("Found var with no val: {}", &def),
                    DataType::Bool  => return Res::new_b(r.b_val),
                    DataType::Int  => return Res::new_i(r.i_val),
                }
            } else {
                panic!("Expected var not defined: {}\n{}", &def, err);
            }
        }
    }
}

fn interp_nary_op(def: &String, args: &Vec<Res>) -> Res {
    for a in args {
        if !matches!(a.data_type, DataType::Int) {
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
    let int_args: Vec<i64> = args.iter().map(|x| x.i_val).collect();

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
            ">"   => return rest.iter().all(|x| first.i_val > x.i_val),
            "<"   => return rest.iter().all(|x| first.i_val < x.i_val),
            "<="  => return rest.iter().all(|x| first.i_val <= x.i_val),
            ">="  => return rest.iter().all(|x| first.i_val >= x.i_val),
            "=="  => return rest.iter().all(|x| first.i_val == x.i_val),
            other =>  panic!("Unrecognised operation: {}", other),
        }
    } else {
        panic!("Missing args for comparison {}, expected more than 2, 0", &def);
    }
}

