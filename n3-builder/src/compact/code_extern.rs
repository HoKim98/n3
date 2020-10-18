use serde::{Deserialize, Serialize};

use super::code::CodeData;
use super::graph::Graphs;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternCode {
    data: CodeData,
}

impl Compact for crate::externs::ExternCode {
    type Output = ExternCode;

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        Self::Output {
            data: self.data.compact(ctx),
        }
    }
}

impl ArrangeId for ExternCode {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.data.arrange_id(ids);
    }
}

impl Decompact for ExternCode {
    type Args = ();
    type Output = crate::externs::ExternCode;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            data: self.data.decompact(ctx, ()),
        }
    }
}
