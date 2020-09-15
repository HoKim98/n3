use crate::ast;
use crate::cache::CloneSafe;
use crate::code::Code;
use crate::error::Result;
use crate::externs::ExternIR;
use crate::graph::Table;
use crate::nodes::{NodeIR, NodeRoot};
use crate::seed::Seed;

#[derive(Debug)]
pub struct TensorGraph(Vec<TensorNode>);

#[derive(Debug)]
pub enum TensorNode {
    Node(NodeIR),
    Extern(ExternIR),
}

impl Into<TensorNode> for NodeIR {
    fn into(self) -> TensorNode {
        TensorNode::Node(self)
    }
}

impl Into<TensorGraph> for Vec<TensorNode> {
    fn into(self) -> TensorGraph {
        TensorGraph(self)
    }
}

impl Into<TensorNode> for ExternIR {
    fn into(self) -> TensorNode {
        TensorNode::Extern(self)
    }
}

impl TensorGraph {
    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        let input_node = &self.0[0];
        if input_node.is_input() {
            input_node.get_output_shapes()
        } else {
            input_node.get_input_shapes()
        }
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        for node in self.0.iter().rev() {
            if let Some(shapes) = node.get_output_shapes() {
                // filter dynamic size
                if shapes.0.len() == 1 {
                    if let Some(None) = shapes.0.get("x") {
                        continue;
                    }
                }
                return Some(shapes);
            }
        }
        self.0.last().unwrap().get_output_shapes()
    }

    pub fn build(self, root: &NodeRoot) -> Result<Vec<Code>> {
        self.0.into_iter().map(|x| x.build(root)).collect()
    }
}

impl TensorNode {
    pub fn is_input(&self) -> bool {
        match self {
            Self::Node(node) => node.data.id == 0,
            Self::Extern(_) => false,
        }
    }

    pub fn name(&self) -> &str {
        todo!()
    }

    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        match self {
            Self::Node(node) => node.get_input_shapes(),
            Self::Extern(node) => node.get_input_shapes(),
        }
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        match self {
            Self::Node(node) => node.get_output_shapes(),
            Self::Extern(node) => node.get_output_shapes(),
        }
    }

    pub fn build(self, root: &NodeRoot) -> Result<Code> {
        match self {
            Self::Node(node) => Ok(node.build(root)?.into()),
            Self::Extern(node) => Ok(node.build(root)?.into()),
        }
    }

    pub fn apply_variables(&mut self, variables: Table) -> Result<()> {
        todo!()
    }
}

impl CloneSafe for TensorNode {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        todo!()
    }
}
