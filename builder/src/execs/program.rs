use std::collections::BTreeMap;

use super::var::Vars;
use crate::graph::Table;
use crate::nodes::NodeIR;

pub type Nodes = BTreeMap<String, NodeIR>;

#[derive(Debug)]
pub struct Program {
    pub graph: Table,
    pub nodes: Nodes,
    pub args: Vars,
}
