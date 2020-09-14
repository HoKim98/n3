use crate::ast;
use crate::code::Code;

#[derive(Clone)]
pub struct NodeCode {
    pub name: String,
    pub input: ast::Outs,
    pub output: ast::Outs,
    pub graph: Vec<Code>,
}
