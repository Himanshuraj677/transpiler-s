use super::stmt::Stmt;

pub struct GateDecl {
    pub name: String,
    pub parameters: Vec<String>,
    pub qubits: Vec<String>,
    pub body: Vec<Stmt>,
}

pub struct OpaqueDecl {
    pub name: String,
    pub parameters: Vec<String>,
    pub qubits: Vec<String>,
}
