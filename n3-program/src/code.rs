use std::collections::BTreeMap;

use crate::ast;
use crate::externs::ExternCode;
use crate::graph::Table;
use crate::nodes::NodeCode;

pub type Codes = BTreeMap<String, Code>;

#[derive(Debug, PartialEq)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CodeType {
    Node,
    Extern,
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

impl Code {
    pub fn ty(&self) -> CodeType {
        match self {
            Self::Node(_) => CodeType::Node,
            Self::Extern(_) => CodeType::Extern,
        }
    }

    pub fn data(&self) -> &CodeData {
        match self {
            Self::Node(node) => &node.data,
            Self::Extern(node) => &node.data,
        }
    }

    pub fn as_node(&self) -> &NodeCode {
        match self {
            Self::Node(node) => node,
            _ => unreachable!("The code should be node."),
        }
    }

    pub fn as_extern(&self) -> &ExternCode {
        match self {
            Self::Extern(node) => node,
            _ => unreachable!("The code should be external."),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CodeData {
    pub name: String,
    pub graph: Table,
    pub input: ast::Outs,
    pub output: ast::Outs,
}
