use super::ir::NodeIR;
use crate::cache::NodeCache;
use crate::error::Result;
use crate::externs::PythonScript;
use crate::n3_std;
use crate::seed::Seed;

pub struct NodeRoot {
    pub(crate) seed: Seed,
    sources: NodeCache<NodeIR>,
    externs: NodeCache<PythonScript>,
    parser: crate::Parser,
}

impl NodeRoot {
    pub fn new() -> Self {
        Self {
            seed: Seed::default(),
            sources: NodeCache::new(n3_std::get_sources()),
            externs: NodeCache::new(n3_std::get_externs()),
            parser: crate::Parser::new(),
        }
    }

    pub fn get_extern(&self, name: &str) -> Result<PythonScript> {
        self.externs.get(name, self)
    }
}
