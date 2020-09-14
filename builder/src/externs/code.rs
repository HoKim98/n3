use super::script::PythonScript;
use crate::ast;

#[derive(Clone)]
pub struct ExternCode {
    pub name: String,
    pub input: ast::Outs,
    pub output: ast::Outs,
    pub script: PythonScript,
}