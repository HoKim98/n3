use std::rc::Rc;

use crate::error::Result;
use crate::externs::PythonScripts;
use crate::nodes::NodeRoot;
use crate::tensor::IRData;

pub use n3_program::code::*;

pub trait AddScripts {
    fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()>;
}

pub trait DataFromIR {
    fn from_ir(data: IRData) -> Self;
}

impl AddScripts for Code {
    fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        match self {
            Self::Node(node) => node.add_scripts(root, scripts),
            Self::Extern(node) => node.add_scripts(root, scripts),
        }
    }
}

impl DataFromIR for CodeData {
    fn from_ir(data: IRData) -> Self {
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
