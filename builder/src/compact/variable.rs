use serde::{Deserialize, Serialize};

use super::value::Value;
use super::{Compact, Context};
use crate::ast;
use crate::error::Result;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Variable {
    pub id: u64,
    pub name: String,
}

pub struct VarAsKey<'a>(pub &'a ast::RefVariable);
pub struct VarAsValue<'a>(pub &'a ast::RefVariable);

impl<'a> Compact for VarAsKey<'a> {
    type Output = (String, Option<Value>);

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        let self_ref = self.0.borrow();
        let name = self_ref.name.clone();
        let value = self_ref.value.compact(ctx)?;
        Ok((name, value))
    }
}

impl<'a> Compact for VarAsValue<'a> {
    type Output = Variable;

    fn compact(&self, _ctx: &mut Context) -> Result<Self::Output> {
        let self_ref = self.0.borrow();
        Ok(Self::Output {
            id: self_ref.id.unwrap(),
            name: self_ref.name.clone(),
        })
    }
}
