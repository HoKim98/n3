use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::graph::Graphs;
use super::variable::{VarAsValue, VariableValue};
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;
use crate::error::Result;

pub type Values = BTreeMap<String, Option<Value>>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    UInt(u64),
    Int(i64),
    Real(f64),
    String(String),
    Dim(ast::OutDim),
    Variable(VariableValue),
    Expr(Box<Expr>),

    List(ValueList),
    Map(ValueMap),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueList(Vec<Value>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueMap(Values);

impl Compact for ast::Value {
    type Output = Value;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(match self {
            Self::Bool(x) => Self::Output::Bool(*x),
            Self::UInt(x) => Self::Output::UInt(*x),
            Self::Int(x) => Self::Output::Int(*x),
            Self::Real(x) => Self::Output::Real(*x),
            Self::String(x) => Self::Output::String(x.clone()),
            Self::Node(_) => crate::variable::node_variable_should_be_pruned(),
            Self::Dim(x) => Self::Output::Dim(x.clone()),
            Self::Variable(x) => Self::Output::Variable(VarAsValue(x).compact(ctx)?),
            Self::Expr(x) => Self::Output::Expr(x.compact(ctx)?),
            Self::List(x) => Self::Output::List(ValueList(x.compact(ctx)?)),
            Self::Map(x) => Self::Output::Map(ValueMap(x.compact(ctx)?)),
        })
    }
}

impl ArrangeId for Value {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        match self {
            Self::Variable(x) => x.arrange_id(ids),
            Self::Expr(x) => x.arrange_id(ids),
            Self::List(x) => x.arrange_id(ids),
            Self::Map(x) => x.arrange_id(ids),
            _ => {}
        }
    }
}

impl Decompact for Value {
    type Args = ();
    type Output = ast::Value;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        match self {
            Self::Bool(x) => Self::Output::Bool(x),
            Self::UInt(x) => Self::Output::UInt(x),
            Self::Int(x) => Self::Output::Int(x),
            Self::Real(x) => Self::Output::Real(x),
            Self::String(x) => Self::Output::String(x),
            Self::Dim(x) => Self::Output::Dim(x),
            Self::Variable(x) => Self::Output::Variable(x.decompact(ctx, ())),
            Self::Expr(x) => Self::Output::Expr(x.decompact(ctx, ())),
            Self::List(x) => Self::Output::List(x.decompact(ctx, ())),
            Self::Map(x) => Self::Output::Map(x.decompact(ctx, ())),
        }
    }
}

impl ArrangeId for ValueList {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for ValueList {
    type Args = ();
    type Output = Vec<ast::Value>;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        self.0.decompact(ctx, ())
    }
}

impl ArrangeId for ValueMap {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for ValueMap {
    type Args = ();
    type Output = crate::graph::Values;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        self.0.decompact(ctx, ())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Expr {
    op: ast::Operator,
    lhs: Value,
    rhs: Option<Value>,
}

impl Compact for ast::Expr {
    type Output = Expr;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Self::Output {
            op: self.op,
            lhs: self.lhs.compact(ctx)?,
            rhs: self.rhs.compact(ctx)?,
        })
    }
}

impl ArrangeId for Expr {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.lhs.arrange_id(ids);
        self.rhs.arrange_id(ids);
    }
}

impl Decompact for Expr {
    type Args = ();
    type Output = ast::Expr;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output {
            op: self.op,
            lhs: self.lhs.decompact(ctx, ()),
            rhs: self.rhs.decompact(ctx, ()),
        }
    }
}
