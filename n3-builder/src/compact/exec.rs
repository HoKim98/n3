use std::rc::Rc;

use serde::{Deserialize, Serialize};

use super::graph::Graph;
use super::ir_extern::Scripts;
use super::tensor::TensorNodes;
use super::{Compact, CompactContext, Decompact, DecompactContext};
use crate::error::Result;
use crate::nodes::NodeRoot;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub(super) graphs: Vec<Graph>,
    pub(super) nodes: TensorNodes,
    pub(super) scripts: Scripts,
}

impl Program {
    pub fn compact(root: &NodeRoot, program: &crate::execs::Program) -> Result<Self> {
        let mut ctx = CompactContext::new(root);

        // note: the args cannot be compacted; the graph will do it instead.
        program.graph.compact(&mut ctx)?;
        ctx.nodes = program.nodes.compact(&mut ctx)?;
        Ok(ctx.build())
    }

    pub fn decompact(self, root: &NodeRoot) -> crate::execs::Program {
        let mut ctx = DecompactContext::new(root, &self.graphs);

        // note: ordered (graphs -> args -> scripts)
        self.graphs.decompact(&mut ctx, ());
        let nodes = self.nodes.decompact(&mut ctx, ());
        self.scripts.decompact(&mut ctx, ());

        let graph = ctx.get_graph(0).clone();
        // the context also has the first graph, so drop it ahead.
        drop(ctx);

        crate::execs::Program {
            graph: Rc::try_unwrap(graph).unwrap().into_inner().into_variables(),
            nodes,
        }
    }
}
