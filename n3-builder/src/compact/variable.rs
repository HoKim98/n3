use serde::{Deserialize, Serialize};

use super::graph::Graphs;
use super::value::Value;
use super::{ArrangeId, Compact, CompactContext, Decompact, DecompactContext};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableKey {
    shortcut: Option<String>,
    ty: Option<ast::LetType>,
    value: Option<Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VariableValue {
    id: u64,
    name: String,
}

pub struct VarAsKey<'a>(pub &'a ast::RefVariable);
pub struct VarAsValue<'a>(pub &'a ast::RefVariable);

impl<'a> Compact for VarAsKey<'a> {
    type Output = (String, VariableKey);

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        let self_ref = self.0.borrow();

        let name = self_ref.name.clone();
        let shortcut = self_ref.shortcut.clone();
        let ty = self_ref.ty.clone();
        let value = self_ref.value.compact(ctx)?;

        Ok((
            name,
            VariableKey {
                shortcut,
                ty,
                value,
            },
        ))
    }
}

impl ArrangeId for VariableKey {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.value.arrange_id(ids)
    }
}

impl Decompact for VariableKey {
    type Args = (u64, String);
    type Output = (ast::RefVariable, Option<Value>);

    fn decompact(self, _ctx: &mut DecompactContext, (id, name): Self::Args) -> Self::Output {
        let mut var = ast::Variable::with_name(name);
        var.id = Some(id);
        var.id_old = Some(id);
        var.shortcut = self.shortcut;
        var.ty = self.ty;

        (var.into(), self.value)
    }
}

impl<'a> Compact for VarAsValue<'a> {
    type Output = VariableValue;

    fn compact(&self, _ctx: &mut CompactContext) -> Result<Self::Output> {
        let self_ref = self.0.borrow();
        Ok(Self::Output {
            id: self_ref.id.unwrap(),
            name: self_ref.name.clone(),
        })
    }
}

impl ArrangeId for VariableValue {
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.id = ids[&self.id];
    }
}

impl Decompact for VariableValue {
    type Args = ();
    type Output = ast::RefVariable;

    fn decompact(self, ctx: &mut DecompactContext, (): Self::Args) -> Self::Output {
        ctx.get_variable(self.id, &self.name)
    }
}
