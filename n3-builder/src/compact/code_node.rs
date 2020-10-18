use serde::{Deserialize, Serialize};

use super::code::{Code, CodeData};
use super::graph::Graphs;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeCode {
    data: CodeData,
    tensor_graph: Vec<Code>,
}

impl Compact for crate::nodes::NodeCode {
    type Output = NodeCode;

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        Self::Output {
            data: self.data.compact(ctx),
            tensor_graph: self.tensor_graph.compact(ctx),
        }
    }
}

impl ArrangeId for NodeCode {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.data.arrange_id(ids);
        self.tensor_graph.arrange_id(ids);
    }
}

impl Decompact for NodeCode {
    type Args = ();
    type Output = crate::nodes::NodeCode;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            data: self.data.decompact(ctx, ()),
            tensor_graph: self.tensor_graph.decompact(ctx, ()),
        }
    }
}
