use std::collections::BTreeMap;

use crate::graph::Table;
use crate::nodes::NodeIR;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub graph: Table,
    pub nodes: BTreeMap<String, NodeIR>,
}
