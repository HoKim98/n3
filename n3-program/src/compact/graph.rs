use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use super::value::Values;
use super::variable::{VarAsKey, VariableKey};
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};

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

impl Decompact for Vec<Table> {
    type Args = ();
    type Output = ();

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        let values = self
            .into_iter()
            .enumerate()
            .map(|(id, graph)| {
                let id = id as u64;
                let (graph, values) = graph.decompact(ctx, id);
                ctx.insert_graph(id, graph);
                (id, values)
            })
            .collect();

        ctx.apply_variables(Graphs(values));
    }
}

impl Graphs<Table> {
    pub fn arrange_id(mut self) -> (Graphs<u64>, Vec<Table>) {
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
pub struct Table(BTreeMap<String, VariableKey>);

impl Deref for Table {
    type Target = BTreeMap<String, VariableKey>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Table {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ArrangeId for Table {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.0.arrange_id(ids)
    }
}

impl Decompact for Table {
    type Args = u64;
    type Output = (crate::graph::Table, Values);

    fn decompact(self, ctx: &mut DecompactContext, id: Self::Args) -> Self::Output {
        let mut variables = BTreeMap::new();
        let mut values = BTreeMap::new();

        for (name, data) in self.0 {
            let (var, value) = data.decompact(ctx, (id, name.clone()));
            variables.insert(name.clone(), var);
            values.insert(name, value);
        }

        let graph = crate::graph::Table { id, variables };
        (graph, values)
    }
}

impl Compact for crate::graph::Table {
    type Output = u64;

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        compact_table(ctx, &self.variables, self.id);
        self.id
    }
}

impl Compact for crate::graph::Variables {
    type Output = ();

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        compact_table(ctx, self, 0)
    }
}

#[derive(Clone, Debug)]
pub struct UncompactedEnv<'a>(pub &'a Option<crate::graph::Values>);

impl<'a> Compact for UncompactedEnv<'a> {
    type Output = ();

    fn compact(&self, ctx: &mut CompactContext) -> Self::Output {
        ctx.env = self.0.as_ref().map(|x| Env(x.compact(ctx)))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env(pub Values);

impl Decompact for Env {
    type Args = ();
    type Output = crate::graph::Values;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        self.0
            .into_iter()
            .map(|(k, v)| (k, v.decompact(ctx, ())))
            .collect()
    }
}

fn compact_table(ctx: &mut CompactContext, variables: &crate::graph::Variables, id: u64) {
    if !ctx.contains_graph(&id) {
        let graph = variables
            .values()
            .map(|var| VarAsKey(var).compact(ctx))
            .collect();
        ctx.insert_graph(id, Table(graph));
    }
}
