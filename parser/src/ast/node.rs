use std::collections::BTreeMap;
use std::fmt;

use super::fmt::FmtGuard;
use super::graph::GraphNode;
use super::variable::{Keywords, NodeLet, Value};

pub struct With {
    pub name: String,
    pub graph: Keywords,
}

crate::impl_debug_no_guard!(With);
impl<'a> fmt::Debug for FmtGuard<'a, With> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}with {}:\n", &indent, &self.name)?;

        for (name, value) in &self.graph {
            self.child(&FmtWithSet { name, value }).fmt(f)?;
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

#[derive(Copy, Clone, PartialEq)]
pub enum NodeType {
    Default,
    Exec,
    Extern(ExternNodeType),
}

impl NodeType {
    pub fn is_extern(&self) -> bool {
        match self {
            Self::Extern(_) => true,
            _ => false,
        }
    }

    pub fn is_exec(&self) -> bool {
        *self == Self::Exec
    }

    pub fn unwrap_extern(self) -> ExternNodeType {
        match self {
            Self::Extern(ty) => ty,
            _ => unreachable!("expected extern type"),
        }
    }
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => Ok(()),
            Self::Exec => write!(f, "exec "),
            Self::Extern(ty) => ty.fmt(f),
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum ExternNodeType {
    Default,
    Data,
    Optim,
}

impl fmt::Debug for ExternNodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "extern "),
            Self::Data => write!(f, "data "),
            Self::Optim => write!(f, "optim "),
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

crate::impl_debug_no_guard!(Node);
impl<'a> fmt::Debug for FmtGuard<'a, Node> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indent = self.indent();
        write!(f, "{}{:?}node {}:\n", &indent, &self.ty, &self.name)?;

        for value in self.graph.values() {
            self.child(value).fmt(f)?;
        }
        for value in self.withs.values() {
            self.child(value).fmt(f)?;
        }
        for value in self.children.values() {
            self.child(value).fmt(f)?;
        }
        for value in self.tensor_graph.values() {
            self.child(value).fmt(f)?;
        }
        Ok(())
    }
}
