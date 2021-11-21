#[derive(Debug)]
pub enum NodeType {
    NaryOp,
    Val,
    Let,
    Seq,
    While,
    If,
    Print,
    Empty
}

// (def ...children)
#[derive(Debug)]
pub struct ASTNode {
    pub node_type: NodeType,
    pub def: String,
    pub children: Vec<ASTNode>
}

#[derive(Debug, Clone)]
pub enum DataType {
    NoRes,
    Bool,
    Int
}

pub struct Res {
    pub data_type: DataType,
    pub b_val: bool,
    pub i_val: i64,
}

impl Res {
    pub fn new() -> Res {
        Res {
            data_type: DataType:: NoRes,
            i_val: 0,
            b_val: false
        }
    }

    pub fn copy(r: &Res) -> Res {
        Res {
            data_type: r.data_type.clone(), 
            i_val: r.i_val, 
            b_val: r.b_val
        }
    }

    pub fn new_i(i: i64) -> Res {
        Res {
            data_type: DataType::Int,
            i_val: i,
            b_val: false
        }
    }

    pub fn new_b(b: bool) -> Res {
        Res {
            data_type: DataType::Bool,
            i_val: 0,
            b_val: b
        }
    }

    pub fn to_string(&self) -> String {
        match self.data_type {
            DataType::NoRes => "NoRes".to_string(),
            DataType::Bool => self.b_val.to_string(),
            DataType::Int => self.i_val.to_string()
        }
    }
}
