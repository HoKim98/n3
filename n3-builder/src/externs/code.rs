use super::script::PythonScripts;
use crate::code::CodeData;
use crate::error::Result;
use crate::nodes::NodeRoot;

#[derive(Debug, PartialEq)]
pub struct ExternCode {
    pub data: CodeData,
}

impl ExternCode {
    pub fn add_script(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        let name = &self.data.name;
        if !scripts.contains_key(name) {
            let script = root.get_extern(name)?;
            scripts.insert(name.clone(), script);
        }
        Ok(())
    }
}
