use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::value::Value;
use super::variable::VarAsKey;
use super::{Compact, Context};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Graphs(BTreeMap<u64, Graph>);

impl Deref for Graphs {
    type Target = BTreeMap<u64, Graph>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graphs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Graph(BTreeMap<String, Option<Value>>);

impl Deref for Graph {
    type Target = BTreeMap<String, Option<Value>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Graph {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Compact for crate::graph::RefGraph {
    type Output = u64;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        self.borrow().compact(ctx)
    }
}

impl Compact for crate::graph::Graph {
    type Output = u64;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        compact_table(ctx, self.variables(), self.id)
    }
}

impl Compact for crate::graph::Table {
    type Output = ();

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        compact_table(ctx, self, 0)?;
        Ok(())
    }
}

fn compact_table(ctx: &mut Context, table: &crate::graph::Table, id: u64) -> Result<u64> {
    let graph = table
        .values()
        .map(|var| VarAsKey(var).compact(ctx))
        .collect::<Result<_>>()?;
    ctx.insert_graph(id, Graph(graph));
    Ok(id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shapes(BTreeMap<String, Option<Shape>>);

impl Compact for ast::Shapes {
    type Output = Shapes;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Shapes(self.0.borrow().compact(ctx)?))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape(Vec<Value>);

impl Compact for ast::Shape {
    type Output = Shape;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Shape(self.0.compact(ctx)?))
    }
}
