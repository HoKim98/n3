use super::exec::Program;
use super::graph::{Graph, Graphs};
use super::ir_extern::Scripts;
use super::tensor::TensorNodes;
use super::value::Values;
use super::{ArrangeId, Decompact};
use crate::ast;
use crate::error::Result;
use crate::nodes::NodeRoot;

pub struct CompactContext<'a> {
    root: &'a NodeRoot,
    graphs: Graphs<Graph>,
    pub nodes: TensorNodes,
    scripts: Scripts,
}

impl<'a> CompactContext<'a> {
    pub fn new(root: &'a NodeRoot) -> Self {
        Self {
            root,
            graphs: Graphs::new(),
            nodes: TensorNodes::new(),
            scripts: Scripts::new(),
        }
    }

    pub fn contains_graph(&mut self, id: &u64) -> bool {
        self.graphs.contains_key(id)
    }

    pub fn insert_graph(&mut self, id: u64, graph: Graph) {
        self.graphs.insert(id, graph);
    }

    pub fn add_script(&mut self, name: &str) -> Result<()> {
        if !self.scripts.contains_key(name) {
            let script = self.root.get_extern(name)?;
            self.scripts.insert(name.to_string(), script);
        }
        Ok(())
    }

    pub fn build(mut self) -> Program {
        let (ids, graphs) = self.graphs.arrange_id();
        self.nodes.arrange_id(&ids);

        Program {
            graphs,
            nodes: self.nodes,
            scripts: self.scripts,
        }
    }
}

pub struct DecompactContext<'a> {
    pub seed: u64,
    root: &'a mut NodeRoot,
    graphs: Graphs<crate::graph::RefGraph>,
}

impl<'a> DecompactContext<'a> {
    pub fn new(root: &'a mut NodeRoot, graphs: &[Graph]) -> Self {
        Self {
            seed: root.seed.alloc(graphs.len() as u64),
            root,
            graphs: Graphs::new(),
        }
    }

    pub fn insert_graph(&mut self, id: u64, graph: crate::graph::RefGraph) {
        self.graphs.insert(id, graph);
    }

    pub fn apply_variables(&mut self, variables: Graphs<Values>) {
        for (id, variables) in variables.0 {
            for (name, value) in variables {
                let value = value.decompact(self, ());
                let graph = self.graphs[&id].borrow();

                graph.get(&name).unwrap().borrow_mut().value = value;
            }
        }
    }

    pub fn add_script(&mut self, name: String, source: String) {
        self.root.add_source(name, source)
    }

    pub fn get_graph(&self, id: u64) -> &crate::graph::RefGraph {
        &self.graphs[&(id + self.seed)]
    }

    pub fn get_variable(&self, id: u64, name: &str) -> ast::RefVariable {
        self.get_graph(id).borrow().get(name).unwrap().clone()
    }
}
