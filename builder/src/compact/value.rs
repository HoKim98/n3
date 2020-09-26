use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::variable::{VarAsValue, Variable};
use super::{Compact, Context};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Value {
    Bool(bool),
    UInt(u64),
    Int(i64),
    Real(f64),
    String(String),
    Dim(ast::OutDim),
    Variable(Variable),
    Expr(Box<Expr>),

    List(ValueList),
    Map(ValueMap),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueList(Vec<Value>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValueMap(BTreeMap<String, Option<Value>>);

impl Compact for ast::Value {
    type Output = Value;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(match self {
            Self::Bool(x) => Self::Output::Bool(*x),
            Self::UInt(x) => Self::Output::UInt(*x),
            Self::Int(x) => Self::Output::Int(*x),
            Self::Real(x) => Self::Output::Real(*x),
            Self::String(x) => Self::Output::String(x.clone()),
            Self::Node(_) => crate::variable::node_variable_should_be_pruned(),
            Self::Dim(x) => Self::Output::Dim(x.clone()),
            Self::Variable(var) => Self::Output::Variable(VarAsValue(var).compact(ctx)?),
            Self::Expr(x) => Self::Output::Expr(x.compact(ctx)?),
            Self::List(x) => Self::Output::List(ValueList(x.compact(ctx)?)),
            Self::Map(x) => Self::Output::Map(ValueMap(x.compact(ctx)?)),
        })
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

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Self::Output {
            op: self.op,
            lhs: self.lhs.compact(ctx)?,
            rhs: self.rhs.compact(ctx)?,
        })
    }
}
