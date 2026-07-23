use super::stmt::Stmt;

pub struct Program {
    pub version: String,
    pub statements: Vec<Stmt>,
}
