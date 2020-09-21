use crate::execs::Program;
use crate::externs::ExternCode;
use crate::nodes::NodeCode;

#[derive(Debug)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
    Exec(Program),
}

impl Into<Code> for NodeCode {
    fn into(self) -> Code {
        Code::Node(self)
    }
}

impl Into<Code> for ExternCode {
    fn into(self) -> Code {
        Code::Extern(self)
    }
}

impl Into<Code> for Program {
    fn into(self) -> Code {
        Code::Exec(self)
    }
}
