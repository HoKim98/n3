use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::value::{Value, Values};
use super::variable::{VarAsKey, VariableKey};
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Graphs<T>(pub BTreeMap<u64, T>);

impl<T> Deref for Graphs<T> {
    type Target = BTreeMap<u64, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Graphs<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Graphs<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }
}

impl Decompact for Vec<Graph> {
    type Args = ();
    type Output = ();

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        let values = self
            .into_iter()
            .enumerate()
            .map(|(id, graph)| {
                let id = id as u64 + ctx.seed;
                let (graph, values) = graph.decompact(ctx, id);
                ctx.insert_graph(id, graph);
                (id, values)
            })
            .collect();

        ctx.apply_variables(Graphs(values));
    }
}

impl Graphs<Graph> {
    pub fn arrange_id(mut self) -> (Graphs<u64>, Vec<Graph>) {
        let ids = Graphs(
            self.0
                .keys()
                .enumerate()
                .map(|(v, k)| (*k, v as u64))
                .collect(),
        );
        self.0.arrange_id(&ids);

        let graphs = self.0.into_iter().map(|(_, x)| x).collect();
        (ids, graphs)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Graph(BTreeMap<String, VariableKey>);

impl Deref for Graph {
    type Target = BTreeMap<String, VariableKey>;

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

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        self.borrow().compact(ctx)
    }
}

impl ArrangeId for Graph {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for Graph {
    type Args = u64;
    type Output = (crate::graph::RefGraph, Values);

    fn decompact(self, ctx: &mut DecompactContext, id: Self::Args) -> Self::Output {
        let mut variables = BTreeMap::new();
        let mut values = BTreeMap::new();

        for (name, data) in self.0 {
            let (var, value) = data.decompact(ctx, (id, name.clone()));
            variables.insert(name.clone(), var);
            values.insert(name, value);
        }

        let graph = crate::graph::Graph::with_variables(id, variables);
        (graph.into(), values)
    }
}

impl Compact for crate::graph::Graph {
    type Output = u64;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        compact_table(ctx, self.variables(), self.id)
    }
}

impl Compact for crate::graph::Table {
    type Output = ();

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        compact_table(ctx, self, 0)?;
        Ok(())
    }
}

fn compact_table(ctx: &mut CompactContext, table: &crate::graph::Table, id: u64) -> Result<u64> {
    if !ctx.contains_graph(&id) {
        let graph = table
            .values()
            .map(|var| VarAsKey(var).compact(ctx))
            .collect::<Result<_>>()?;
        ctx.insert_graph(id, Graph(graph));
    }
    Ok(id)
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shapes(BTreeMap<String, Option<Shape>>);

impl Compact for ast::Shapes {
    type Output = Shapes;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Shapes(self.0.borrow().compact(ctx)?))
    }
}

impl ArrangeId for Shapes {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for Shapes {
    type Args = ();
    type Output = ast::Shapes;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        Self::Output::new(self.0.decompact(ctx, ()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Shape(Vec<Value>);

impl Compact for ast::Shape {
    type Output = Shape;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Shape(self.0.compact(ctx)?))
    }
}

impl ArrangeId for Shape {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for Shape {
    type Args = ();
    type Output = ast::Shape;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        ast::Shape(self.0.decompact(ctx, ()))
    }
}
