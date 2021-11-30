#[derive(Debug, Clone)]
pub enum NodeType {
    NaryOp,
    Val,
    Let,
    Seq,
    While,
    If,
    Import,
    List,
    BuiltinFunction,
    Function,
    Empty
}

// (def ...children)
#[derive(Debug, Clone)]
pub struct ASTNode {
    pub node_type: NodeType,
    pub def: String,
    pub children: Vec<ASTNode>
}

#[derive(Debug, Clone)]
pub enum Res {
    NoRes,
    Bool(bool),
    Int(i64),
    Str(String),
    List(Vec<Res>),
    Func(ASTNode)
}

impl Res {
    pub fn to_string(&self) -> String {
        match &self {
            Res::NoRes => "NoRes".to_string(),
            Res::Bool(b) => b.to_string(),
            Res::Int(i) => i.to_string(),
            Res::Func(f) => format!("{:?}", f),
            Res::Str(s) => s.clone(),
            Res::List(v) => {
                let mut v_clone = v.clone();
                v_clone.reverse();
                let mut out: String = String::new();
                for i in 0..v_clone.len() {
                    if i == 0 {
                        out.push_str(&v_clone[i].to_string());
                    } else {
                        out.push_str(&format!(" {}", v_clone[i].to_string()));
                    }
                }
                return format!("[{}]", out);
            }
        }
    }
}
