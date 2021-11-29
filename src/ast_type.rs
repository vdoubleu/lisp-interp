#[derive(Debug, Clone)]
pub enum NodeType {
    NaryOp,
    Val,
    Let,
    Seq,
    While,
    If,
    Import,
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
    Func(ASTNode)
}

impl Res {
    pub fn to_string(&self) -> String {
        match &self {
            Res::NoRes => "NoRes".to_string(),
            Res::Bool(b) => b.to_string(),
            Res::Int(i) => i.to_string(),
            Res::Func(f) => format!("{:?}", f),
            Res::Str(s) => s.clone()
        }
    }
}
