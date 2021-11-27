#[derive(Debug, Clone)]
pub enum NodeType {
    NaryOp,
    Val,
    Let,
    Seq,
    While,
    If,
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

    /*
    pub fn get_b(&self) -> bool {
        match self.data_type {
            DataType::Bool(b) => b,
            _ => panic!("Trying to retrieve bool, but is not bool"),
        }
    }
    */

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
