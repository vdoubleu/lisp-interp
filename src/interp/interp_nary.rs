use crate::ast_type::Res;

pub fn interp_nary_op(def: &String, args: &Vec<Res>) -> Res {
    match def.as_ref() {
        "+" => {
            if check_res_vec_all_type(&args, Res::Int(0)) {
                return Res::Int(int_nary_op(&def, &args));
            } else if check_res_vec_all_type(&args, Res::Str("".to_string())) {
                return Res::Str(str_nary_op(&def, &args));
            }
            panic!("Expected int type or string type in arg, for variable but instead got something else: {:?}", &args);
        },
        "-" | "*" | "/"   => {
            if !check_res_vec_all_type(&args, Res::Int(0)) {
                panic!("Expected int type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::Int(int_nary_op(&def, &args));
        },
        "==" | "!=" => {
            return Res::Bool(equality_nary_op(&def, &args));
        },
        ">" | ">=" | "<" | "<=" => {
            if !check_res_vec_all_type(&args, Res::Int(0)) {
                panic!("Expected int type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::Bool(bool_nary_op(&def, &args));
        },
        "||" | "&&" => {
            if !check_res_vec_all_type(&args, Res::Bool(false)) {
                panic!("Expected bool type in arg, for variable but instead got something else: {:?}", &args);
            }
            return Res::Bool(logic_nary_op(&def, &args));
        },
        _                       => panic!("Unrecognised nary op: {}", &def),
    }
}

fn check_res_vec_all_type(v: &Vec<Res>, typ: Res) -> bool {
    return v.iter()
            .all(|x| {
                if std::mem::discriminant(x) == std::mem::discriminant(&typ) {
                    return true;
                } else {
                    return false;
                }
            });
}

fn str_nary_op(def: &String, args: &Vec<Res>) -> String {
    let string_args: Vec<String> = args.iter().map(|x| {
        match &x {
            Res::Str(i) => i.clone(),
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
        match x {
            Res::Bool(b) => *b,
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
        match x {
            Res::Int(i) => *i,
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

fn equality_nary_op(def: &String, args: &Vec<Res>) -> bool {
    let mut int_args: Vec<i64> = Vec::new();
    let mut string_args: Vec<String> = Vec::new();
    let mut bool_args: Vec<bool> = Vec::new();

    for arg in args {
        match arg {
            Res::Int(i) => int_args.push(*i),
            Res::Str(s) => string_args.push(s.clone()),
            Res::Bool(b) => bool_args.push(*b),
            _ => panic!("unexpected type in equality op"),
        }
    }

    match def.as_ref() {
        "==" => {
            if int_args.len() == args.len() {
                return int_args.iter().all(|x| *x == *int_args.first().unwrap());
            } else if string_args.len() == args.len() {
                return string_args.iter().all(|x| *x == *string_args.first().unwrap());
            } else if bool_args.len() == args.len() {
                return bool_args.iter().all(|x| *x == *bool_args.first().unwrap());
            } else {
                panic!("unexpected type in equality op");
            }
        },
        "!=" => {
            if int_args.len() == args.len() {
                return int_args.iter().any(|x| *x != *int_args.first().unwrap());
            } else if string_args.len() == args.len() {
                return string_args.iter().any(|x| *x != *string_args.first().unwrap());
            } else if bool_args.len() == args.len() {
                return bool_args.iter().any(|x| *x != *bool_args.first().unwrap());
            } else {
                panic!("unexpected type in inequality op");
            }
        },
        other => panic!("Unrecognised operation: {}", other),
    }
}

fn bool_nary_op(def: &String, args: &Vec<Res>) -> bool {
    fn get_int_or_panic(r: &Res) -> i64 {
        match r {
            Res::Int(i) => *i,
            _ => panic!("Expected int in bool nary op"),
        }
    }

    if let Some((first, rest)) = args.split_first() {
        if let Res::Int(first_i) = first {
            match def.as_ref() {
                ">"   => return rest.iter().all(|x| *first_i > get_int_or_panic(x)),
                "<"   => return rest.iter().all(|x| *first_i < get_int_or_panic(x)),
                "<="  => return rest.iter().all(|x| *first_i <= get_int_or_panic(x)),
                ">="  => return rest.iter().all(|x| *first_i >= get_int_or_panic(x)),
                other =>  panic!("Unrecognised operation: {}", other),
            }
        } else {
            panic!("Expected ints in nary op");
        }
    } else {
        panic!("Missing args for comparison {}, expected more than 2, 0", &def);
    }
}

