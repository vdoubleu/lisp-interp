mod type_huh;

use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    Res,
    ASTNode
};
use crate::interp::interp_builtin::type_huh::type_huh;
use crate::reader::interp_args::InterpArgs;
use std::collections::HashMap;


pub fn interp_builtin(def: &String, args: &Vec<ASTNode>, store: &mut Vec<HashMap<String, Res>>, interp_args: &InterpArgs) -> Res {
    match def.as_str() {
        "print" => {
            let mut s = String::new();
            for arg in args {
                s.push_str(interp_ast(arg, store, interp_args).to_string().as_str());
            }
            print!("{}", s);
            return Res::Str(s);
        },
        "println" => {
            let mut s = String::new();
            for arg in args {
                s.push_str(interp_ast(arg, store, interp_args).to_string().as_str());
            }
            println!("{}", s);
            return Res::Str(s);
        },
        "read" => {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).expect("Failed to read line");
            return Res::Str(s);
        },
        "len" => {
            if args.len() != 1 {
                panic!("len takes exactly one argument");
            }

            let arg = args[0].clone();
            let interp_res = interp_ast(&arg, store, interp_args);
            if let Res::Str(s) = interp_res {
                return Res::Int(s.len() as i64);
            } else if let Res::List(l) = interp_res {
                return Res::Int(l.len() as i64);
            } else {
                panic!("len takes a string or a list");
            }
        },
        "str" => {
            let mut s = String::new();
            for arg in args {
                s.push_str(interp_ast(arg, store, interp_args).to_string().as_str());
            }
            return Res::Str(s);
        },
        "num" => {
            let mut s = String::new();
            for arg in args {
                let r = interp_ast(arg, store, interp_args);
                match r {
                    Res::Int(i) => s.push_str(i.to_string().as_str()),
                    Res::Str(st) => s.push_str(st.to_string().as_str()),
                    Res::Bool(b) => s.push_str(if b { "1" } else { "0" }),
                    _ => panic!("num takes an argument that can be converted to a number, the following cannot: {}", r.to_string())
                }
            }
            return Res::Int(s.parse::<i64>().unwrap());
        },
        "bool" => {
            let mut s = String::new();
            for arg in args {
                let r = interp_ast(arg, store, interp_args);
                match r {
                    Res::Int(i) => s.push_str(if i == 0 { "false" } else { "true" }),
                    Res::Str(st) => s.push_str(&st),
                    Res::Bool(b) => s.push_str(if b { "true" } else { "false" }),
                    _ => panic!("bool takes an argument that can be converted to a boolean, the following cannot: {}", r.to_string())
                }
            }
            return Res::Bool(s.parse::<bool>().unwrap());
        },
        "list" => {
            if args.len() == 1 {
                let r = interp_ast(&args[0], store, interp_args);
                
                if let Res::Str(s) = r {
                    let mut l: Vec<Res> = s.chars().map(|c| Res::Str(c.to_string())).collect();
                    l.reverse();
                    return Res::List(l);
                } else {
                    panic!("list type conversion only supports a string as an argument");
                }
            } else {
                panic!("list type conversion takes one argument");
            }
        },
        "string?" => type_huh(&String::from("string?"), args, store, Res::Str(String::from("")), interp_args), 
        "num?" => type_huh(&String::from("num?"), args, store, Res::Int(0), interp_args), 
        "bool?" => type_huh(&String::from("bool?"), args, store, Res::Bool(false), interp_args),
        "func?" => type_huh(&String::from("func?"), args, store, Res::Func(ASTNode::default()), interp_args),
        "list?" => type_huh(&String::from("list?"), args, store, Res::List(vec![]), interp_args),
        "shell" => {
            let mut s = String::new();
            for arg in args {
                s.push_str(&(interp_ast(arg, store, interp_args).to_string() + " "));
            }
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg(s.as_str())
                .output()
                .expect("failed to execute process");
                
            println!("{}\n", &s);
            println!("{}", String::from_utf8_lossy(&output.stdout));
            let out = Res::Str(String::from_utf8(output.stdout).unwrap());
            let err = Res::Str(String::from_utf8(output.stderr).unwrap());

            return Res::List(vec![err, out]);
        },
        "first" => {
            if args.len() != 1 {
                panic!("first takes exactly one argument");
            }

            let arg = args[0].clone();
            let interp_res = interp_ast(&arg, store, interp_args);
            if let Res::List(l) = interp_res {
                if l.len() == 0 {
                    return Res::List(vec![]);
                } else {
                    return l[l.len() - 1].clone();
                }
            } else {
                panic!("first takes a list");
            }
        },
        "rest" => {
            if args.len() != 1 {
                panic!("rest takes exactly one argument");
            }

            let arg = args[0].clone();
            let interp_res = interp_ast(&arg, store, interp_args);
            if let Res::List(l) = interp_res {
                if l.len() == 0 {
                    return Res::List(vec![]);
                } else {
                    return Res::List(l[..l.len() - 1].to_vec());
                }
            } else {
                panic!("rest takes a list");
            }
        },
        "cons" => {
            if args.len() != 2 {
                panic!("cons takes exactly two arguments");
            }

            let arg1 = args[0].clone();
            let arg2 = args[1].clone();
            let interp_res1 = interp_ast(&arg1, store, interp_args);
            let interp_res2 = interp_ast(&arg2, store, interp_args);
            if let Res::List(l2) = interp_res2 {
                if let Res::NoRes = interp_res1 {
                    // first res is empty, so (cons none l2) = l2
                    return Res::List(l2);
                }

                let mut out = l2.clone();
                out.push(interp_res1);
                return Res::List(out);
            } else {
                panic!("cons second arg must be a list: {}", interp_res2.to_string());
            }
        },
        _ => {
            panic!("Unknown builtin function: {}", def);
        }
    }
}
