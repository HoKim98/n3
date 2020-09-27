use super::script::PythonScript;
use crate::ast;
use crate::graph::RefGraph;

#[derive(Debug, PartialEq)]
pub struct ExternCode {
    pub name: String,
    pub graph: RefGraph,
    pub input: ast::Outs,
    pub output: ast::Outs,
    pub script: PythonScript,
}
