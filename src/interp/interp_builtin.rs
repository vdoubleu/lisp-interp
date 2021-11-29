use crate::interp::interp_ast::interp_ast;
use crate::ast_type::{
    Res,
    ASTNode
};
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
            } else {
                panic!("len takes a string argument, the following is not a string: {}", interp_res.to_string());
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
            return Res::Str(String::from_utf8(output.stdout).unwrap());
        },
        _ => {
            panic!("Unknown builtin function: {}", def);
        }
    }
}
