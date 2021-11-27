use crate::ast_type::Res;
use std::collections::HashMap;

pub fn interp_val(def: &String, store: &Vec<HashMap<String, Res>>, permit_func: bool) -> Res {
    if let Some(i) = val_is_int(&def) {
        return Res::Int(i);
    } else if let Some(b) = val_is_bool(&def) {
        return Res::Bool(b);
    } else if let Some(s) = val_is_string(&def) {
        return Res::Str(s);
    } else if let Some(v) = val_is_var(&def, &store, permit_func) {
        return v;
    } else {
        panic!("Variable not define: {}", &def);
    }
}

fn val_is_int(def: &String) -> Option<i64> {
    return match def.parse::<i64>() {
        Ok(n) => Some(n),
        Err(_) => None
    }
}

fn val_is_bool(def: &String) -> Option<bool> {
    return if def == "true" || def == "false" { Some(def == "true") } else { None };
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
            }
        } 
    }

    return None
}

fn val_is_string(def: &String) -> Option<String> {
    return if def.chars().last().unwrap() == '"' && def.chars().next().unwrap() == '"' {
        Some((&def[1..def.to_string().len()-1]).to_string())
    } else {
        None
    };
}
