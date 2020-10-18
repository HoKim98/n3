use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::ast;
use crate::error::Result;
use crate::graph::RefGraph;
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::TensorNode;

pub type NodeName = Vec<String>;
type NodeNameRef<'a> = &'a [String];

pub struct Context<'a> {
    pub root: &'a NodeRoot,
    parent: BTreeMap<NodeName, RefGraph>,
    children: BTreeMap<NodeName, BTreeMap<String, TensorNode>>,
    uses: BTreeMap<String, TensorNode>,
}

impl<'a> Context<'a> {
    pub fn new(root: &'a NodeRoot) -> Self {
        Context {
            root,
            parent: Default::default(),
            children: Default::default(),
            uses: Default::default(),
        }
    }

    pub fn add_graph(&mut self, name: NodeName, graph: RefGraph) {
        self.parent.insert(name.clone(), graph);
        self.children.insert(name, Default::default());
    }

    pub fn add_child(&mut self, name: NodeNameRef, child: TensorNode) {
        let node_name = child.name().to_string();
        self.children
            .get_mut(name)
            .unwrap()
            .insert(node_name, child);
    }

    pub fn get(&mut self, parent: NodeNameRef, name: &str) -> Result<TensorNode> {
        let mut parent = parent.to_vec();
        while !parent.is_empty() {
            if let Some(node) = self.children.get_mut(&parent).unwrap().get(name) {
                let mut variables = vec![];
                return Ok(node.clone_safe(&self.root.seed, &mut variables));
            }
            parent.pop();
        }

        if let Some(node) = self.uses.get(name) {
            let mut variables = vec![];
            Ok(node.clone_safe(&self.root.seed, &mut variables))
        } else {
            Ok(self.root.get(name)?.into())
        }
    }
}

pub trait Build: CloneSafe {
    type Output: CloneSafe;

    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self::Output>
    where
        Self: Sized;
}

pub trait CloneSafe {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self;
}

impl<T> CloneSafe for Rc<T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Rc::new((**self).clone_safe(seed, variables))
    }
}

impl<T> CloneSafe for RefCell<T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        RefCell::new(self.borrow().clone_safe(seed, variables))
    }
}

impl<T> CloneSafe for BTreeMap<String, T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        self.iter()
            .map(|(k, v)| (k.clone(), v.clone_safe(seed, variables)))
            .collect()
    }
}

impl<T> CloneSafe for Vec<T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        self.iter().map(|x| x.clone_safe(seed, variables)).collect()
    }
}

impl<T> CloneSafe for Option<T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        self.as_ref().map(|x| x.clone_safe(seed, variables))
    }
}

impl<T> CloneSafe for Box<T>
where
    T: CloneSafe,
{
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Box::new((**self).clone_safe(seed, variables))
    }
}
