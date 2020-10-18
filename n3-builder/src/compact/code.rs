use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::code_extern::ExternCode;
use super::code_node::NodeCode;
use super::graph::Graphs;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;

pub type Codes = BTreeMap<String, Code>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Code {
    Node(NodeCode),
    Extern(ExternCode),
}

impl Compact for crate::code::Code {
    type Output = Code;

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        match self {
            Self::Node(x) => Self::Output::Node(x.compact(ctx)),
            Self::Extern(x) => Self::Output::Extern(x.compact(ctx)),
        }
    }
}

impl ArrangeId for Code {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        match self {
            Self::Node(x) => x.arrange_id(ids),
            Self::Extern(x) => x.arrange_id(ids),
        }
    }
}

impl Decompact for Code {
    type Args = ();
    type Output = crate::code::Code;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        match self {
            Self::Node(x) => Self::Output::Node(x.decompact(ctx, ())),
            Self::Extern(x) => Self::Output::Extern(x.decompact(ctx, ())),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CodeData {
    pub name: String,
    pub graph: u64,
    pub input: ast::Outs,
    pub output: ast::Outs,
}

impl Compact for crate::code::CodeData {
    type Output = CodeData;

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        Self::Output {
            name: self.name.clone(),
            graph: self.graph.compact(ctx),
            input: self.input.clone(),
            output: self.output.clone(),
        }
    }
}

impl ArrangeId for CodeData {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.graph = ids[&self.graph];
    }
}

impl Decompact for CodeData {
    type Args = ();
    type Output = crate::code::CodeData;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            name: self.name,
            graph: ctx.get_graph(self.graph).clone(),
            input: self.input,
            output: self.output,
        }
    }
}
