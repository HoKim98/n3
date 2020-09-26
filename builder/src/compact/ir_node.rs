use serde::{Deserialize, Serialize};

use super::tensor::{IRData, TensorGraph};
use super::value::Value;
use super::{Compact, Context};
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

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Self::Output {
            data: self.data.compact(ctx)?,
            ty: self.ty,
            tensor_graph: self.tensor_graph.compact(ctx)?,
            repeat: self.repeat.compact(ctx)?,
        })
    }
}
