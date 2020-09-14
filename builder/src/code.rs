use crate::externs::ExternCode;
use crate::nodes::NodeCode;

#[derive(Clone)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
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
