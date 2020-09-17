use super::code::NodeCode;
use super::root::NodeRoot;
use crate::ast;
use crate::context::{Build, CloneSafe};
use crate::error::Result;
use crate::seed::Seed;
use crate::tensor::{IRData, TensorGraph, TensorNode};

#[derive(Debug)]
pub struct NodeIR {
    pub data: IRData,
    pub tensor_graph: TensorGraph,
    pub repeat: Option<ast::Value>,
}

impl NodeIR {
    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        self.tensor_graph.get_input_shapes()
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        self.tensor_graph.get_output_shapes()
    }

    pub fn build(self, root: &NodeRoot) -> Result<NodeCode> {
        if let Some(repeat) = self.repeat {
            todo!()
        }

        let tensor_graph = self.tensor_graph.build(root)?;

        Ok(NodeCode {
            name: self.data.name,
            input: self.data.input,
            output: self.data.output,
            graph: tensor_graph,
        })
    }
}

impl Build for NodeIR {
    type Output = TensorNode;

    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self::Output> {
        TensorNode::build(root, name, source)
    }
}

impl CloneSafe for NodeIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        // note: ordered (data -> tensor_graph -> repeat)
        let mut cloned = Self {
            data: self.data.clone_safe(seed, variables),
            tensor_graph: self.tensor_graph.clone_safe(seed, variables),
            repeat: self.repeat.clone_safe(seed, variables),
        };

        // note: the ExternIR wrapper's graph should be cloned manually.
        if let Some(mut node) = cloned.tensor_graph.try_borrow_mut_extern_node() {
            node.data.graph = cloned.data.graph.clone();
        }
        cloned
    }
}
