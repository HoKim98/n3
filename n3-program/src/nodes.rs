use crate::code::{Code, CodeData};

#[derive(Debug, PartialEq)]
pub struct NodeCode {
    pub data: CodeData,
    pub tensor_graph: Vec<Code>,
}
