use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast;
use crate::error::Result;
use crate::externs::{ExternCode, PythonScripts};
use crate::graph::Table;
use crate::nodes::{NodeCode, NodeRoot};
use crate::tensor::IRData;

pub type Codes = BTreeMap<String, Code>;

#[derive(Debug, PartialEq)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
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
    pub fn data(&self) -> &CodeData {
        match self {
            Self::Node(node) => &node.data,
            Self::Extern(node) => &node.data,
        }
    }

    pub(crate) fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        match self {
            Self::Node(node) => node.add_scripts(root, scripts),
            Self::Extern(node) => node.add_script(root, scripts),
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

impl CodeData {
    pub(crate) fn from_ir(data: IRData) -> Self {
        Self {
            name: data.name,
            graph: Rc::try_unwrap(data.graph)
                .unwrap()
                .into_inner()
                .into_table(),
            input: data.input,
            output: data.output,
        }
    }
}
