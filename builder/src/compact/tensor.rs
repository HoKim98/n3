use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::ir_extern::ExternIR;
use super::ir_node::NodeIR;
use super::{Compact, Context};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TensorGraph(Vec<TensorNode>);

impl Compact for crate::tensor::TensorGraph {
    type Output = TensorGraph;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(TensorGraph((**self).compact(ctx)?))
    }
}

pub type TensorNodes = BTreeMap<String, TensorNode>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TensorNode {
    Node(NodeIR),
    Extern(ExternIR),
}

impl Compact for crate::tensor::TensorNode {
    type Output = TensorNode;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(match self {
            Self::Node(x) => Self::Output::Node(x.compact(ctx)?),
            Self::Extern(x) => Self::Output::Extern(x.compact(ctx)?),
            Self::Exec(_) => unreachable!("The exec node cannot be compacted using TensorNode."),
        })
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

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Self::Output {
            id: self.id,
            name: self.name.clone(),
            graph: self.graph.compact(ctx)?,
            input: self.input.clone(),
            output: self.output.clone(),
        })
    }
}
