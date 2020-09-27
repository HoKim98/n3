use std::collections::BTreeMap;

use crate::graph::Table;
use crate::nodes::NodeIR;

type Map<T> = BTreeMap<String, T>;

pub type Nodes = Map<NodeIR>;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub graph: Table,
    pub nodes: Nodes,
}
