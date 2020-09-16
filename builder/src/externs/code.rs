use super::script::PythonScript;
use crate::ast;

#[derive(Clone)]
pub struct ExternCode {
    pub name: String,
    pub input: Option<ast::Outs>,
    pub output: Option<ast::Outs>,
    pub script: PythonScript,
}
