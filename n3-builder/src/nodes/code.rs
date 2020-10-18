use super::root::NodeRoot;
use crate::code::{Code, CodeData};
use crate::error::Result;
use crate::externs::PythonScripts;

#[derive(Debug, PartialEq)]
pub struct NodeCode {
    pub data: CodeData,
    pub tensor_graph: Vec<Code>,
}

impl NodeCode {
    pub fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        for node in &self.tensor_graph {
            node.add_scripts(root, scripts)?;
        }
        Ok(())
    }
}
