use std::cell::UnsafeCell;
use std::path::PathBuf;

use super::ir::NodeIR;
use crate::cache::NodeCache;
use crate::error::Result;
use crate::execs::{ExecIR, GlobalVars};
use crate::externs::PythonScript;
use crate::n3_std;
use crate::seed::Seed;
use crate::tensor::TensorNode;

pub struct NodeRoot {
    pub(crate) seed: Seed,
    sources: NodeCache<TensorNode>,
    externs: NodeCache<PythonScript>,
    pub(crate) parser: crate::Parser,
    _thread_unsafe: UnsafeCell<()>,
}

impl NodeRoot {
    pub fn new(n3_source_root: Option<&str>) -> Self {
        let n3_source_root = n3_source_root
            .map(PathBuf::from)
            .unwrap_or_else(GlobalVars::get_n3_source_root);

        Self {
            seed: Seed::default(),
            sources: NodeCache::new(n3_std::get_sources(&n3_source_root)),
            externs: NodeCache::new(n3_std::get_externs(&n3_source_root)),
            parser: crate::Parser::default(),
            _thread_unsafe: UnsafeCell::new(()),
        }
    }
}

impl NodeRoot {
    pub fn add_source(&self, name: String, source: String) {
        self.sources.add_source(name, source);
    }

    pub fn add_source_path(&self, name: String, path: String) {
        self.sources.add_path(name, path);
    }

    pub fn add_extern_path(&self, name: String, path: String) {
        self.externs.add_path(name, path);
    }

    pub(crate) fn get(&self, name: &str) -> Result<NodeIR> {
        self.sources.get(name, self)?.unwrap_node()
    }

    pub(crate) fn get_exec(&self, name: &str) -> Result<ExecIR> {
        self.sources.get(name, self)?.unwrap_exec()
    }

    pub(crate) fn get_extern(&self, name: &str) -> Result<PythonScript> {
        self.externs.get(name, self)
    }
}

#[cfg(feature = "pip")]
impl Drop for NodeRoot {
    fn drop(&mut self) {
        unsafe { n3_torch_ffi::finalize_python() }
    }
}
