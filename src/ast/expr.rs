pub enum UnaryOp {
    Plus,
    Minus,
}

pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

pub enum Expr {
    Integer(i64),
    Real(i64),
    Pi,
    Identifier(String),
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}
