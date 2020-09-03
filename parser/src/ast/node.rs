use std::collections::HashMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::graph::GraphNode;
use super::variable::{NodeLet, Value};

pub struct With {
    pub name: String,
    pub graph: HashMap<String, Value>,
}

impl<'a> fmt::Debug for FmtGuard<'a, With> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}with {}:\n", &indent, &self.name)?;

        for value in self.graph.values() {
            self.resolve(value).fmt(f)?;
        }
        Ok(())
    }
}

pub enum NodeType {
    Default,
    Extern,
    Data,
    Optim,
    Exec,
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Extern => write!(f, "extern "),
            Self::Data => write!(f, "data "),
            Self::Optim => write!(f, "optim "),
            Self::Exec => write!(f, "exec "),
        }
    }
}

pub struct Node {
    pub name: String,
    pub ty: NodeType,

    pub graph: HashMap<String, NodeLet>,
    pub withs: HashMap<String, With>,
    pub children: HashMap<String, Node>,
    pub tensor_graph: HashMap<u64, GraphNode>,
}

impl<'a> fmt::Debug for FmtGuard<'a, Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}{:?}node {}:\n", &indent, &self.ty, &self.name)?;

        for value in self.graph.values() {
            self.resolve(value).fmt(f)?;
        }
        for value in self.withs.values() {
            self.resolve(value).fmt(f)?;
        }
        for value in self.children.values() {
            self.resolve(value).fmt(f)?;
        }
        for value in self.tensor_graph.values() {
            self.resolve(value).fmt(f)?;
        }
        Ok(())
    }
}
