use crate::ast;
use crate::code::Code;
use crate::graph::RefGraph;

#[derive(Debug, PartialEq)]
pub struct NodeCode {
    pub name: String,
    pub graph: RefGraph,
    pub input: ast::Outs,
    pub output: ast::Outs,
    pub tensor_graph: Vec<Code>,
}
