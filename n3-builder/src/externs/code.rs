use super::script::PythonScripts;
use crate::code::AddScripts;
use crate::error::Result;
use crate::nodes::NodeRoot;

pub use n3_program::externs::ExternCode;

impl AddScripts for ExternCode {
    fn add_scripts(&self, root: &NodeRoot, scripts: &mut PythonScripts) -> Result<()> {
        let name = &self.data.name;
        if !scripts.contains_key(name) {
            let script = root.get_extern(name)?;
            scripts.insert(name.clone(), script);
        }
        Ok(())
    }
}
