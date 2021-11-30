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

#[test]
fn test_check_res_vec_all_type() {
    let v = vec![Res::Int(0), Res::Int(0), Res::Int(0)];
    assert_eq!(check_res_vec_all_type(&v, Res::Int(0)), true);
    let v = vec![Res::Int(0), Res::Int(0), Res::Str("".to_string())];
    assert_eq!(check_res_vec_all_type(&v, Res::Int(0)), false);
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

#[test]
fn test_str_nary_op() {
    assert_eq!(str_nary_op(&"+".to_string(), &vec![Res::Str("a".to_string()), Res::Str("b".to_string())]), "ab".to_string());
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

#[test]
fn test_logic_nary_op() {
    assert_eq!(logic_nary_op(&"||".to_string(), &vec![Res::Bool(true), Res::Bool(false)]), true);
    assert_eq!(logic_nary_op(&"&&".to_string(), &vec![Res::Bool(true), Res::Bool(false)]), false);
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

#[test]
fn test_int_nary_op() {
    assert_eq!(int_nary_op(&"+".to_string(), &vec![Res::Int(1), Res::Int(2), Res::Int(3)]), 6);
    assert_eq!(int_nary_op(&"-".to_string(), &vec![Res::Int(1), Res::Int(2), Res::Int(3)]), -4);
    assert_eq!(int_nary_op(&"*".to_string(), &vec![Res::Int(1), Res::Int(2), Res::Int(3)]), 6);
    assert_eq!(int_nary_op(&"/".to_string(), &vec![Res::Int(4), Res::Int(2), Res::Int(3)]), 0);
}

fn equality_nary_op(def: &String, args: &Vec<Res>) -> bool {
    let mut int_first: i64 = 0;
    let mut string_first: String = "".to_string();
    let mut bool_first: bool = false;

    let mut int_args: Vec<i64> = Vec::new();
    let mut string_args: Vec<String> = Vec::new();
    let mut bool_args: Vec<bool> = Vec::new();

    let mut is_first = true;

    for arg in args {
        if is_first {
            match arg {
                Res::Int(i) => int_first = *i,
                Res::Str(s) => string_first = s.clone(),
                Res::Bool(b) => bool_first = *b,
                _ => panic!("expected int, string or bool in equality op"),
            }
            is_first = false;
        } else {
            match arg {
                Res::Int(i) => int_args.push(*i),
                Res::Str(s) => string_args.push(s.clone()),
                Res::Bool(b) => bool_args.push(*b),
                _ => panic!("unexpected type in equality op"),
            }
        }  
    }

    match def.as_ref() {
        "==" => {
            if int_args.len() == args.len() - 1 {
                return int_args.iter().all(|x| *x == int_first);
            } else if string_args.len() == args.len() - 1 {
                return string_args.iter().all(|x| *x == string_first);
            } else if bool_args.len() == args.len() - 1 {
                return bool_args.iter().all(|x| *x == bool_first);
            } else {
                panic!("unexpected type in equality op");
            }
        },
        "!=" => {
            if int_args.len() == args.len() - 1 {
                return int_args.iter().all(|x| *x != int_first);
            } else if string_args.len() == args.len() - 1 {
                return string_args.iter().all(|x| *x != string_first);
            } else if bool_args.len() == args.len() - 1 {
                return bool_args.iter().all(|x| *x != bool_first);
            } else {
                panic!("unexpected type in inequality op");
            }
        },
        other => panic!("Unrecognised operation: {}", other),
    }
}

#[test]
fn test_equality_nary_op() {
    assert_eq!(equality_nary_op(&"==".to_string(), &vec![Res::Int(1), Res::Int(1)]), true);
    assert_eq!(equality_nary_op(&"==".to_string(), &vec![Res::Int(1), Res::Int(2)]), false);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Int(1), Res::Int(1)]), false);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Int(1), Res::Int(2)]), true);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Str("a".to_string()), Res::Str("a".to_string())]), false);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Str("a".to_string()), Res::Str("b".to_string())]), true);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Bool(true), Res::Bool(true)]), false);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Bool(true), Res::Bool(false)]), true);
    assert_eq!(equality_nary_op(&"!=".to_string(), &vec![Res::Bool(true), Res::Bool(false), Res::Bool(true)]), false);
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

#[test]
fn test_bool_nary_op() {
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(1), Res::Int(2)]), false);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(2), Res::Int(1)]), true);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(1), Res::Int(1)]), false);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(2), Res::Int(2)]), false);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(2), Res::Int(1), Res::Int(1)]), true);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(1), Res::Int(2), Res::Int(1)]), false);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(1), Res::Int(1), Res::Int(2)]), false);
    assert_eq!(bool_nary_op(&">".to_string(), &vec![Res::Int(2), Res::Int(2), Res::Int(2)]), false);
    assert_eq!(bool_nary_op(&">=".to_string(), &vec![Res::Int(1), Res::Int(2)]), false);
    assert_eq!(bool_nary_op(&">=".to_string(), &vec![Res::Int(2), Res::Int(1)]), true);
    assert_eq!(bool_nary_op(&">=".to_string(), &vec![Res::Int(1), Res::Int(1)]), true);
}
