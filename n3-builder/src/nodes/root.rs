use std::cell::UnsafeCell;

use super::ir::NodeIR;
use crate::cache::NodeCache;
use crate::error::Result;
use crate::execs::ExecIR;
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

impl Default for NodeRoot {
    fn default() -> Self {
        Self {
            seed: Seed::default(),
            sources: NodeCache::new(n3_std::get_sources()),
            externs: NodeCache::new(n3_std::get_externs()),
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

    pub fn get(&self, name: &str) -> Result<NodeIR> {
        self.sources.get(name, self)?.unwrap_node()
    }

    pub fn get_exec(&self, name: &str) -> Result<ExecIR> {
        self.sources.get(name, self)?.unwrap_exec()
    }

    pub fn get_extern(&self, name: &str) -> Result<PythonScript> {
        self.externs.get(name, self)
    }
}
