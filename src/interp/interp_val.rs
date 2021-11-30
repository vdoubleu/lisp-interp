use crate::ast_type::Res;
use crate::interp::importer::{
    get_stdlib,
    import_module
};
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;

pub fn interp_val(def: &String, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs, permit_func: bool) -> Res {
    let first_attempt = interp_val_splitter(def, store, permit_func);

    if first_attempt != Res::NoRes {
        return first_attempt
    } else if let Some(lib) = get_stdlib(&def) {
        import_module(&lib, store, &interp_args);
        let second_attempt = interp_val_splitter(def, store, permit_func);
        if second_attempt != Res::NoRes {
            return second_attempt;
        } 
    }
    panic!("Could not find value for {}", def);
}

fn interp_val_splitter(def: &String, store: &Vec<HashMap<String, Res>>, permit_func: bool) -> Res {
    if let Some(i) = val_is_int(&def) {
        return Res::Int(i);
    } else if let Some(b) = val_is_bool(&def) {
        return Res::Bool(b);
    } else if let Some(s) = val_is_string(&def) {
        return Res::Str(s);
    } else if let Some(v) = val_is_var(&def, &store, permit_func) {
        return v;
    } else {
        return Res::NoRes;
    }
}

fn val_is_int(def: &String) -> Option<i64> {
    return match def.parse::<i64>() {
        Ok(n) => Some(n),
        Err(_) => None
    }
}

#[test]
fn test_val_is_int() {
    assert_eq!(val_is_int(&String::from("1")), Some(1));
    assert_eq!(val_is_int(&String::from("-1")), Some(-1));
    assert_eq!(val_is_int(&String::from("0")), Some(0));
    assert_eq!(val_is_int(&String::from("-0")), Some(0));
    assert_eq!(val_is_int(&String::from("1.0")), None);
    assert_eq!(val_is_int(&String::from("one")), None);
    assert_eq!(val_is_int(&String::from("1hello2")), None);  
    assert_eq!(val_is_int(&String::from("123")), Some(123));
}

fn val_is_bool(def: &String) -> Option<bool> {
    return if def == "true" || def == "false" { Some(def == "true") } else { None };
}

#[test]
fn test_val_is_bool() {
    assert_eq!(val_is_bool(&String::from("true")), Some(true));
    assert_eq!(val_is_bool(&String::from("false")), Some(false));
    assert_eq!(val_is_bool(&String::from("1")), None);
}

fn val_is_var(def: &String, store: &Vec<HashMap<String, Res>>, permit_func: bool) -> Option<Res> {
    for s in store.iter().rev() {
        if s.contains_key(def) {
            let r: Res = Res::clone(&s[def]);
            match r {
                Res::NoRes => panic!("Found var with no val: {}", &def),
                Res::Bool(b)  => return Some(Res::Bool(b)),
                Res::Int(i)   => return Some(Res::Int(i)),
                Res::Str(s)   => return Some(Res::Str(s)),
                Res::Func(fb) => return if permit_func {
                        Some(Res::Func(fb))
                    } else {
                        panic!("first class functions not supported");
                    },
                Res::List(l) => return Some(Res::List(l)),
            }
        } 
    }

    return None
}

#[test]
fn test_val_is_var() {
    let mut store: Vec<HashMap<String, Res>> = Vec::new();
    let mut s: HashMap<String, Res> = HashMap::new();
    s.insert(String::from("a"), Res::Int(1));
    store.push(s);
    s = HashMap::new();
    s.insert(String::from("b"), Res::Int(2));
    store.push(s);
    assert_eq!(val_is_var(&String::from("a"), &store, false), Some(Res::Int(1)));
    assert_eq!(val_is_var(&String::from("b"), &store, false), Some(Res::Int(2)));
    assert_eq!(val_is_var(&String::from("c"), &store, false), None);
}

fn val_is_string(def: &String) -> Option<String> {
    return if def.chars().last().unwrap() == '"' && def.chars().next().unwrap() == '"' {
        Some((&def[1..def.to_string().len()-1]).to_string())
    } else {
        None
    };
}

#[test]
fn test_val_is_string() {
    assert_eq!(val_is_string(&String::from("\"hello\"")), Some(String::from("hello")));
    assert_eq!(val_is_string(&String::from("\"hello world\"")), Some(String::from("hello world")));
    assert_eq!(val_is_string(&String::from("\"hello\\\"world\"")), Some(String::from("hello\\\"world")));
    assert_eq!(val_is_string(&String::from("\"   hello  \" world  \"")), Some(String::from("   hello  \" world  ")));
    assert_eq!(val_is_string(&String::from("\"   hello  \" world  ")), None);
    assert_eq!(val_is_string(&String::from("   hello  \" \" world  ")), None);
    assert_eq!(val_is_string(&String::from("  hello world  ")), None);
}
