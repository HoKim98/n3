mod context;
mod exec;
mod graph;
mod ir_extern;
mod ir_node;
mod tensor;
mod value;
mod variable;

use std::collections::BTreeMap;

use crate::error::Result;

pub use self::context::Context;
pub use self::exec::Program;

pub trait Compact {
    type Output;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output>;
}

impl<K, V> Compact for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Compact,
{
    type Output = BTreeMap<K, V::Output>;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        self.iter()
            .map(|(k, v)| Ok((k.clone(), v.compact(ctx)?)))
            .collect()
    }
}

impl<T> Compact for Vec<T>
where
    T: Compact,
{
    type Output = Vec<T::Output>;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        self.iter().map(|x| x.compact(ctx)).collect()
    }
}

impl<T> Compact for Option<T>
where
    T: Compact,
{
    type Output = Option<T::Output>;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        match self {
            Some(x) => Ok(Some(x.compact(ctx)?)),
            None => Ok(None),
        }
    }
}

impl<T> Compact for Box<T>
where
    T: Compact,
{
    type Output = Box<T::Output>;

    fn compact(&self, ctx: &mut Context) -> Result<Self::Output> {
        Ok(Self::Output::new((**self).compact(ctx)?))
    }
}
