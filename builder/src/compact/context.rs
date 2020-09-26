use super::exec::Program;
use super::graph::Graph;
use crate::error::Result;
use crate::nodes::NodeRoot;

pub struct Context<'a> {
    root: &'a NodeRoot,
    program: Program,
}

impl<'a> Context<'a> {
    pub fn new(root: &'a NodeRoot) -> Self {
        Self {
            root,
            program: Default::default(),
        }
    }

    pub fn insert_graph(&mut self, id: u64, graph: Graph) {
        self.program.graphs.insert(id, graph);
    }

    pub fn add_script(&mut self, name: &str) -> Result<()> {
        if !self.program.scripts.contains_key(name) {
            let script = self.root.get_extern(name)?;
            self.program.scripts.insert(name.to_string(), script);
        }
        Ok(())
    }

    pub fn build(self) -> Program {
        self.program
    }
}
