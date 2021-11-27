use crate::ast_type::{
    NodeType,
    ASTNode
};

pub fn build_ast(tokens: &Vec<String>) -> Option<ASTNode> {
    let mut childrens_to_complete: Vec<ASTNode> = Vec::new();
    
    let mut is_def = false;
    for t in tokens {
        match t.as_ref() {
            "(" => {
                let new_node = ASTNode { node_type: NodeType::Empty, def: "".to_string(), children: Vec::new() };
                childrens_to_complete.push(new_node);
                is_def = true;
            },
            ")" => {
                if childrens_to_complete.len() > 1 {
                    if let Some(completed_child) = childrens_to_complete.pop() {
                        match childrens_to_complete.last_mut() {
                            Some(l) => l.children.push(completed_child),
                            None    => panic!("Length less than 1??"),
                        }
                    } else {
                        panic!("no last child found??");
                    }
                }
            }
            "+" | "-" | "*" | "/"  | ">" | "<" | "<=" | ">=" | "==" | "!=" | "&&" | "||" => {
                set_last_to_ast_type(&mut childrens_to_complete, 
                                     NodeType::NaryOp, 
                                     t.to_string());
                is_def = false;
                }
            "seq" => {
                set_last_to_ast_type(&mut childrens_to_complete, 
                                      NodeType::Seq,
                                      t.to_string());
                is_def = false;
            }
            "let" => {
                set_last_to_ast_type(&mut childrens_to_complete, 
                                     NodeType::Let,
                                     t.to_string());
                is_def = false;
            }
            "while" => {
                set_last_to_ast_type(&mut childrens_to_complete,
                                     NodeType::While,
                                     t.to_string());
                is_def = false;
            }
            "if" => {
                set_last_to_ast_type(&mut childrens_to_complete,
                                     NodeType::If,
                                     t.to_string());
                is_def = false;
            }
            "print" | "println" | "read" | "len" | "num" | "str" | "bool" => {
                set_last_to_ast_type(&mut childrens_to_complete,
                                     NodeType::BuiltinFunction,
                                     t.to_string());
                is_def = false;
            }
            val => {
                if val.chars().next().unwrap() == '_' {
                    panic!("variable names cannot begin with an underscore: {}", val);
                }

                // construct variables in ast
                if is_def {
                    set_last_to_ast_type(&mut childrens_to_complete, 
                                         NodeType::Function,
                                         t.to_string());
                    is_def = false;
                } else { // variable constants
                    let new_node = ASTNode { node_type: NodeType::Val, def: val.to_string(), children: Vec::new() };
                    match childrens_to_complete.last_mut() {
                        Some(l) => l.children.push(new_node),
                        None    => childrens_to_complete.push(new_node),
                    }
                }
            }
        }
    }
    
    return childrens_to_complete.pop();
}

fn set_last_to_ast_type(childrens_to_complete: &mut Vec<ASTNode>, typ: NodeType, def: String) {
    match childrens_to_complete.last_mut() {
        Some(l) => match l.node_type {
            NodeType::Empty => {
                l.node_type = typ;
                l.def = def;
            },
            _               => panic!("Node already has type, setting type again in args??: {}", def)
        }
        _       => panic!("No body for operators??: {}", def),
    }
}
