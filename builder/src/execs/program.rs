use std::collections::BTreeMap;

use super::var::Vars;
use crate::compact::{Compact, Context, Program as CompactProgram};
use crate::error::Result;
use crate::graph::Table;
use crate::nodes::{NodeIR, NodeRoot};

type Map<T> = BTreeMap<String, T>;

pub type Nodes = Map<NodeIR>;

#[derive(Debug)]
pub struct Program {
    pub graph: Table,
    pub nodes: Nodes,
    pub args: Vars,
}

impl Program {
    pub(crate) fn compact(&self, root: &NodeRoot) -> Result<CompactProgram> {
        let mut ctx = Context::new(root);

        // note: the args cannot be compacted; the graph is already did.
        self.graph.compact(&mut ctx)?;
        self.nodes.compact(&mut ctx)?;
        Ok(ctx.build())
    }

    pub(crate) fn decompact(root: &NodeRoot, compacted: CompactProgram) -> Result<Self> {
        todo!()
    }
}
