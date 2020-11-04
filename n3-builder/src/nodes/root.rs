use std::borrow::Cow;
use std::cell::UnsafeCell;
use std::path::PathBuf;

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

impl NodeRoot {
    pub fn new(n3_source_root: Option<&str>) -> Self {
        let n3_source_root = n3_source_root
            .map(|x| PathBuf::from(x))
            .unwrap_or_else(Self::get_n3_source_root);

        Self {
            seed: Seed::default(),
            sources: NodeCache::new(n3_std::get_sources(&n3_source_root)),
            externs: NodeCache::new(n3_std::get_externs(&n3_source_root)),
            parser: crate::Parser::default(),
            _thread_unsafe: UnsafeCell::new(()),
        }
    }

    #[cfg(feature = "pip")]
    fn get_n3_source_root() -> PathBuf {
        use pyo3::prelude::*;

        use n3_torch_ffi::pyo3;

        Python::with_gil(|py| {
            py.run("import n3", None, None)
                .and_then(|()| py.eval("n3.__file__", None, None))
                .map(|x| x.str().unwrap())
                .map(|x| x.to_string())
                .map(PathBuf::from)
                .map(|mut x| {
                    x.pop(); // remove __init__.py
                    x
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);
                })
                .expect("variable 'N3_SOURCE_ROOT' is incorrect")
        })
    }

    #[cfg(not(feature = "pip"))]
    fn get_n3_source_root() -> PathBuf {
        panic!(
            "variable 'N3_SOURCE_ROOT' is not given. Please set feature pip=true to search automatically."
        )
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
