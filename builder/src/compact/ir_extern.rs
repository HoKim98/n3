use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::graph::Shapes;
use super::tensor::IRData;
use super::{Compact, Context};
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

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        ctx.add_script(&self.data.name)?;
        Ok(Self::Output {
            data: self.data.compact(ctx)?,
            shapes: self.shapes.compact(ctx)?,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternIRShapes {
    pub input: Option<Shapes>,
    pub output: Option<Shapes>,
}

impl Compact for crate::externs::ExternIRShapes {
    type Output = ExternIRShapes;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Self::Output {
            input: self.input.compact(ctx)?,
            output: self.output.compact(ctx)?,
        })
    }
}
