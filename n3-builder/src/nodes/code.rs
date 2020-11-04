use super::root::NodeRoot;
use crate::code::AddScripts;
use crate::error::Result;
use crate::externs::PythonScripts;

pub use n3_program::nodes::NodeCode;

impl AddScripts for NodeCode {
    fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        for node in &self.tensor_graph {
            node.add_scripts(root, scripts)?;
        }
        Ok(())
    }
}
