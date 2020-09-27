use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::graph::Graphs;
use super::ir_extern::ExternIR;
use super::ir_node::NodeIR;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TensorGraph(Vec<TensorNode>);

impl Compact for crate::tensor::TensorGraph {
    type Output = TensorGraph;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(TensorGraph((**self).compact(ctx)?))
    }
}

impl ArrangeId for TensorGraph {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for TensorGraph {
    type Args = ();
    type Output = crate::tensor::TensorGraph;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output::new(self.0.decompact(ctx, ()))
    }
}

pub type TensorNodes = BTreeMap<String, NodeIR>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TensorNode {
    Node(NodeIR),
    Extern(ExternIR),
}

impl Compact for crate::tensor::TensorNode {
    type Output = TensorNode;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(match self {
            Self::Node(x) => Self::Output::Node(x.compact(ctx)?),
            Self::Extern(x) => Self::Output::Extern(x.compact(ctx)?),
            Self::Exec(_) => unreachable!("The exec node cannot be compacted using TensorNode."),
        })
    }
}

impl ArrangeId for TensorNode {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        match self {
            Self::Node(x) => x.arrange_id(ids),
            Self::Extern(x) => x.arrange_id(ids),
        }
    }
}

impl Decompact for TensorNode {
    type Args = ();
    type Output = crate::tensor::TensorNode;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        match self {
            Self::Node(x) => Self::Output::Node(x.decompact(ctx, ())),
            Self::Extern(x) => Self::Output::Extern(x.decompact(ctx, ())),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IRData {
    pub id: u64,
    pub name: String,
    pub graph: u64,
    pub input: ast::Outs,
    pub output: ast::Outs,
}

impl Compact for crate::tensor::IRData {
    type Output = IRData;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Self::Output {
            id: self.id,
            name: self.name.clone(),
            graph: self.graph.compact(ctx)?,
            input: self.input.clone(),
            output: self.output.clone(),
        })
    }
}

impl ArrangeId for IRData {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.graph = ids[&self.graph];
    }
}

impl Decompact for IRData {
    type Args = ();
    type Output = crate::tensor::IRData;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            id: self.id,
            name: self.name.clone(),
            graph: ctx.get_graph(self.graph).clone(),
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}
