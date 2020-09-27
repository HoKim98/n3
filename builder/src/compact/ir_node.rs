use serde::{Deserialize, Serialize};

use super::graph::Graphs;
use super::tensor::{IRData, TensorGraph};
use super::value::Value;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeIR {
    data: IRData,
    ty: ast::LetNodeType,
    tensor_graph: TensorGraph,
    repeat: Option<Value>,
}

impl Compact for crate::nodes::NodeIR {
    type Output = NodeIR;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Self::Output {
            data: self.data.compact(ctx)?,
            ty: self.ty,
            tensor_graph: self.tensor_graph.compact(ctx)?,
            repeat: self.repeat.compact(ctx)?,
        })
    }
}

impl ArrangeId for NodeIR {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.data.arrange_id(ids);
        self.tensor_graph.arrange_id(ids);
        self.repeat.arrange_id(ids);
    }
}

impl Decompact for NodeIR {
    type Args = ();
    type Output = crate::nodes::NodeIR;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            data: self.data.decompact(ctx, ()),
            ty: self.ty,
            tensor_graph: self.tensor_graph.decompact(ctx, ()),
            repeat: self.repeat.decompact(ctx, ()),
        }
    }
}
