mod context;
mod exec;
mod graph;
mod ir_extern;
mod ir_node;
mod tensor;
mod value;
mod variable;

use std::collections::BTreeMap;

use self::graph::Graphs;
use crate::error::Result;

pub use self::context::{CompactContext, DecompactContext};
pub use self::exec::Program;

pub trait Compact {
    type Output;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output>;
}

pub trait ArrangeId {
    fn arrange_id(&mut self, ids: &Graphs<u64>);
}

pub trait Decompact {
    type Args;
    type Output;

    fn decompact(self, ctx: &mut DecompactContext, args: Self::Args) -> Self::Output;
}

impl<K, V> Compact for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: Compact,
{
    type Output = BTreeMap<K, V::Output>;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        self.iter()
            .map(|(k, v)| Ok((k.clone(), v.compact(ctx)?)))
            .collect()
    }
}

impl<K, V> ArrangeId for BTreeMap<K, V>
where
    K: Clone + Ord,
    V: ArrangeId,
{
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.values_mut().for_each(|x| x.arrange_id(ids))
    }
}

impl<K, V> Decompact for BTreeMap<K, V>
where
    K: Ord,
    V: Decompact<Args = ()>,
{
    type Args = V::Args;
    type Output = BTreeMap<K, V::Output>;

    fn decompact(self, ctx: &mut DecompactContext, args: Self::Args) -> Self::Output {
        self.into_iter()
            .map(|(k, v)| (k, v.decompact(ctx, args)))
            .collect()
    }
}

impl<T> Compact for Vec<T>
where
    T: Compact,
{
    type Output = Vec<T::Output>;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        self.iter().map(|x| x.compact(ctx)).collect()
    }
}

impl<T> ArrangeId for Vec<T>
where
    T: ArrangeId,
{
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        self.iter_mut().for_each(|x| x.arrange_id(ids))
    }
}

impl<T> Decompact for Vec<T>
where
    T: Decompact<Args = ()>,
{
    type Args = ();
    type Output = Vec<T::Output>;

    fn decompact(self, ctx: &mut DecompactContext, args: Self::Args) -> Self::Output {
        self.into_iter().map(|x| x.decompact(ctx, args)).collect()
    }
}

impl<T> Compact for Option<T>
where
    T: Compact,
{
    type Output = Option<T::Output>;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        match self {
            Some(x) => Ok(Some(x.compact(ctx)?)),
            None => Ok(None),
        }
    }
}

impl<T> ArrangeId for Option<T>
where
    T: ArrangeId,
{
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        if let Some(x) = self.as_mut() {
            x.arrange_id(ids);
        }
    }
}

impl<T> Decompact for Option<T>
where
    T: Decompact,
{
    type Args = T::Args;
    type Output = Option<T::Output>;

    fn decompact(self, ctx: &mut DecompactContext, args: Self::Args) -> Self::Output {
        self.map(|x| x.decompact(ctx, args))
    }
}

impl<T> Compact for Box<T>
where
    T: Compact,
{
    type Output = Box<T::Output>;

    fn compact(&self, ctx: &mut CompactContext) -> Result<Self::Output> {
        Ok(Self::Output::new((**self).compact(ctx)?))
    }
}

impl<T> ArrangeId for Box<T>
where
    T: ArrangeId,
{
    fn arrange_id(&mut self, ids: &Graphs<u64>) {
        (**self).arrange_id(ids)
    }
}

impl<T> Decompact for Box<T>
where
    T: Decompact,
{
    type Args = T::Args;
    type Output = Box<T::Output>;

    fn decompact(self, ctx: &mut DecompactContext, args: Self::Args) -> Self::Output {
        Self::Output::new((*self).decompact(ctx, args))
    }
}
