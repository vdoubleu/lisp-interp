#[derive(Debug, Clone)]
pub enum NodeType {
    NaryOp,
    Val,
    Let,
    Seq,
    While,
    If,
    Print,
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
pub enum DataType {
    NoRes,
    Bool(bool),
    Int(i64),
    Str(String),
    Func(ASTNode)
}

#[derive(Debug)]
pub struct Res {
    pub data_type: DataType,
}

impl Res {
    pub fn new() -> Res {
        Res {
            data_type: DataType:: NoRes,
        }
    }

    pub fn copy(r: &Res) -> Res {
        Res {
            data_type: r.data_type.clone(), 
        }
    }

    pub fn new_i(i: i64) -> Res {
        Res {
            data_type: DataType::Int(i),
        }
    }

    pub fn new_b(b: bool) -> Res {
        Res {
            data_type: DataType::Bool(b),
        }
    }

    pub fn new_f(f: &ASTNode) -> Res {
        Res {
            data_type: DataType::Func(f.clone()),
        }
    }

    pub fn new_s(s: &String) -> Res {
        Res {
            data_type: DataType::Str(s.clone()),
        }
    }

    pub fn get_i(&self) -> i64 {
        match self.data_type {
            DataType::Int(i) => i,
            _ => panic!("Trying to retrieve int, but is not int"),
        }
    }

    /*
    pub fn get_b(&self) -> bool {
        match self.data_type {
            DataType::Bool(b) => b,
            _ => panic!("Trying to retrieve bool, but is not bool"),
        }
    }
    */

    pub fn to_string(&self) -> String {
        match &self.data_type {
            DataType::NoRes => "NoRes".to_string(),
            DataType::Bool(b) => b.to_string(),
            DataType::Int(i) => i.to_string(),
            DataType::Func(f) => format!("{:?}", f),
            DataType::Str(s) => s.clone()
        }
    }
}
