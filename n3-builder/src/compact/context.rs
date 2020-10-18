use super::code::Codes;
use super::exec::Program;
use super::graph::{Graphs, Table};
use super::value::Values;
use super::{ArrangeId, Decompact};
use crate::ast;
use crate::externs::PythonScripts;

pub struct CompactContext {
    graphs: Graphs<Table>,
    pub nodes: Codes,
    scripts: PythonScripts,
}

impl CompactContext {
    pub fn new(scripts: PythonScripts) -> Self {
        Self {
            graphs: Graphs::new(),
            nodes: Codes::new(),
            scripts,
        }
    }

    pub fn contains_graph(&mut self, id: &u64) -> bool {
        self.graphs.contains_key(id)
    }

    pub fn insert_graph(&mut self, id: u64, graph: Table) {
        self.graphs.insert(id, graph);
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

pub struct DecompactContext {
    graphs: Graphs<crate::graph::Table>,
}

impl DecompactContext {
    pub fn new() -> Self {
        Self {
            graphs: Graphs::new(),
        }
    }

    pub fn insert_graph(&mut self, id: u64, graph: crate::graph::Table) {
        self.graphs.insert(id, graph);
    }

    pub fn apply_variables(&mut self, variables: Graphs<Values>) {
        for (id, variables) in variables.0 {
            for (name, value) in variables {
                let value = value.decompact(self, ());
                let graph = &mut self.get_graph_mut(id).variables;

                graph.get(&name).unwrap().borrow_mut().value = value;
            }
        }
    }

    pub fn get_graph(&self, id: u64) -> &crate::graph::Table {
        &self.graphs[&id]
    }

    fn get_graph_mut(&mut self, id: u64) -> &mut crate::graph::Table {
        self.graphs.get_mut(&id).unwrap()
    }

    pub fn get_variable(&self, id: u64, name: &str) -> ast::RefVariable {
        self.get_graph(id).variables.get(name).unwrap().clone()
    }
}
