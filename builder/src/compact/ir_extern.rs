use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::graph::{Graphs, Shapes};
use super::tensor::IRData;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::error::Result;

pub type Scripts = BTreeMap<String, Script>;
pub type Script = crate::externs::PythonScript;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternIR {
    data: IRData,
    shapes: ExternIRShapes,
}

impl Compact for crate::externs::ExternIR {
    type Output = ExternIR;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        ctx.add_script(&self.data.name)?;
        Ok(Self::Output {
            data: self.data.compact(ctx)?,
            shapes: self.shapes.compact(ctx)?,
        })
    }
}

impl ArrangeId for ExternIR {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.data.arrange_id(ids);
        self.shapes.arrange_id(ids);
    }
}

impl Decompact for ExternIR {
    type Args = ();
    type Output = crate::externs::ExternIR;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            data: self.data.decompact(ctx, ()),
            shapes: self.shapes.decompact(ctx, ()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternIRShapes {
    pub input: Option<Shapes>,
    pub output: Option<Shapes>,
}

impl Compact for crate::externs::ExternIRShapes {
    type Output = ExternIRShapes;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Self::Output {
            input: self.input.compact(ctx)?,
            output: self.output.compact(ctx)?,
        })
    }
}

impl ArrangeId for ExternIRShapes {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.input.arrange_id(ids);
        self.output.arrange_id(ids);
    }
}

impl Decompact for ExternIRShapes {
    type Args = ();
    type Output = crate::externs::ExternIRShapes;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            input: self.input.decompact(ctx, ()),
            output: self.output.decompact(ctx, ()),
        }
    }
}

impl Decompact for Script {
    type Args = ();
    type Output = ();

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        ctx.add_script(self.name, self.source)
    }
}
