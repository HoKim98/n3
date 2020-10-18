use std::io;

use serde::{Deserialize, Serialize};

use super::code::Codes;
use super::graph::Table;
use super::{Compact, CompactContext, Decompact, DecompactContext};
use crate::error::Result;
use crate::externs::PythonScripts;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Program {
    pub(super) graphs: Vec<Table>,
    pub(super) nodes: Codes,
    pub(super) scripts: PythonScripts,
}

impl Program {
    pub(crate) fn compact(program: &crate::execs::Program) -> Self {
        let mut ctx = CompactContext::new(program.scripts.clone());

        // note: the args cannot be compacted; the graph will do it instead.
        program.graph.compact(&mut ctx);
        ctx.nodes = program.nodes.compact(&mut ctx);
        ctx.build()
    }

    pub fn decompact(self) -> crate::execs::Program {
        let mut ctx = DecompactContext::new();

        // note: ordered (graphs -> args)
        self.graphs.decompact(&mut ctx, ());
        let nodes = self.nodes.decompact(&mut ctx, ());

        let graph = ctx.get_graph(0).clone().variables;
        // the context also has the first graph, so drop it ahead.
        drop(ctx);

        crate::execs::Program {
            graph,
            nodes,
            scripts: self.scripts,
        }
    }

    pub fn save<W>(&self, writer: W) -> Result<()>
    where
        W: io::Write,
    {
        bincode::serialize_into(writer, self).map_err(|e| e.into())
    }

    pub(crate) fn load<R>(reader: R) -> Result<Program>
    where
        R: io::Read,
    {
        let program: Self = bincode::deserialize_from(reader)?;
        Ok(program)
    }
}
