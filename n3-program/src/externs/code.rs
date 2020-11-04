use crate::ast::ExternNodeType;
use crate::code::CodeData;

#[derive(Debug, PartialEq)]
pub struct ExternCode {
    pub ty: ExternNodeType,
    pub data: CodeData,
}
