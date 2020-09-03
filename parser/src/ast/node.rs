use std::collections::BTreeMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::graph::GraphNode;
use super::variable::{NodeLet, Value};

pub struct With {
    pub name: String,
    pub graph: BTreeMap<String, Value>,
}

impl<'a> fmt::Debug for FmtGuard<'a, With> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}with {}:\n", &indent, &self.name)?;

        for (name, value) in &self.graph {
            self.resolve(&FmtWithSet { name, value }).fmt(f)?;
        }
        Ok(())
    }
}

struct FmtWithSet<'a> {
    name: &'a str,
    value: &'a Value,
}

impl<'a> fmt::Debug for FmtGuard<'a, FmtWithSet<'a>> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}set {} = {:?}\n", &indent, &self.name, &self.value)
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

    pub graph: BTreeMap<String, NodeLet>,
    pub withs: BTreeMap<String, With>,
    pub children: BTreeMap<String, Node>,
    pub tensor_graph: BTreeMap<u64, GraphNode>,
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
