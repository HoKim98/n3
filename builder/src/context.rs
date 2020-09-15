use std::collections::BTreeMap;

use crate::ast;
use crate::error::Result;
use crate::graph::RefGraph;
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::TensorNode;

pub type NodeName = Vec<String>;

pub struct Context<'a> {
    pub root: &'a NodeRoot,
    parent: BTreeMap<NodeName, RefGraph>,
    uses: BTreeMap<String, TensorNode>,
}

impl<'a> Context<'a> {
    pub fn new(root: &'a NodeRoot) -> Self {
        Context {
            root,
            parent: Default::default(),
            uses: Default::default(),
        }
    }

    pub fn add_child(&mut self, name: NodeName, child: RefGraph) {
        self.parent.insert(name, child);
    }

    pub fn get(&mut self, name: &str) -> Result<TensorNode> {
        if let Some(node) = self.uses.get(name) {
            let mut variables = vec![];
            Ok(node.clone_safe(&self.root.seed, &mut variables))
        } else {
            Ok(self.root.get(name)?.into())
        }
    }
}

pub trait Build: CloneSafe {
    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self>
    where
        Self: Sized;
}

pub trait CloneSafe {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self;
}
