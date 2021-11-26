use crate::ast_type::{
    Res,
    DataType,
};

pub fn interp_nary_op(def: &String, args: &Vec<Res>) -> Res {
    match def.as_ref() {
        "+" => {
            if check_res_vec_all_type(&args, DataType::Int(0)) {
                return Res::new_i(int_nary_op(&def, &args));
            } else if check_res_vec_all_type(&args, DataType::Str("".to_string())) {
                return Res::new_s(&str_nary_op(&def, &args));
            }
            panic!("Expected int type or string type in arg, for variable but instead got something else: {:?}", &args);
        },
        "-" | "*" | "/"   => {
            if !check_res_vec_all_type(&args, DataType::Int(0)) {
                panic!("Expected int type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::new_i(int_nary_op(&def, &args));
        },
        "==" | "!=" => {
            //if check_res_vec_all_type(&args, DataType::Bool(false)) {
            //    return Res::new_b(bool_nary_op(&def, &args));
            //} else 
            if check_res_vec_all_type(&args, DataType::Int(0)) {
                return Res::new_b(bool_nary_op(&def, &args));
            }
            
            panic!("Expected int type, bool type, or string type in arg, for variable but instead got something else: {:?}", &args);
        },
        ">" | ">=" | "<" | "<=" => {
            if !check_res_vec_all_type(&args, DataType::Int(0)) {
                panic!("Expected int type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::new_b(bool_nary_op(&def, &args));
        },
        "||" | "&&" => {
            if !check_res_vec_all_type(&args, DataType::Bool(false)) {
                panic!("Expected bool type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::new_b(logic_nary_op(&def, &args));
        },
        _                       => panic!("Unrecognised nary op: {}", &def),
    }
}

fn check_res_vec_all_type(v: &Vec<Res>, typ: DataType) -> bool {
    return v.iter()
            .all(|x| {
                if std::mem::discriminant(&x.data_type) == std::mem::discriminant(&typ) {
                    return true;
                } else {
                    return false;
                }
            });
}

fn str_nary_op(def: &String, args: &Vec<Res>) -> String {
    let string_args: Vec<String> = args.iter().map(|x| {
        match &x.data_type {
            DataType::Str(i) => i.clone(),
            _ => panic!("expected string in nary op"),
        }
    }).collect();

    match def.as_ref() {
        "+" => return string_args.join(&"".to_string()),
        _ => panic!("Unrecognised string nary op: {}", &def),
    }
}

fn logic_nary_op(def: &String, args: &Vec<Res>) -> bool {
    let bool_args: Vec<bool> = args.iter().map(|x| {
        match x.data_type {
            DataType::Bool(b) => b,
            _ => panic!("expected bool in logic op"),
        }
    }).collect();

    match def.as_ref() {
        "||" => bool_args.iter().any(|x| *x),
        "&&" => bool_args.iter().all(|x| *x),
        other => panic!("unexpected logic operation: {}", other),
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
            "!="  => return rest.iter().all(|x| first.get_i() != x.get_i()),
            other =>  panic!("Unrecognised operation: {}", other),
        }
    } else {
        panic!("Missing args for comparison {}, expected more than 2, 0", &def);
    }
}

