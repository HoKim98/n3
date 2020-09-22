use crate::execs::Program;
use crate::externs::ExternCode;
use crate::nodes::NodeCode;

#[derive(Debug)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
    Exec(Program),
}

impl From<NodeCode> for Code {
    fn from(code: NodeCode) -> Self {
        Self::Node(code)
    }
}

impl From<ExternCode> for Code {
    fn from(code: ExternCode) -> Self {
        Self::Extern(code)
    }
}

impl From<Program> for Code {
    fn from(code: Program) -> Self {
        Self::Exec(code)
    }
}
