use super::node::NodeEntry;
use crate::ast;
use crate::error::Result;

pub struct GraphNodeEntry<'a, 'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
    pub root: &'c mut NodeEntry<'a, 'b>,
    pub id: u64,
    pub node: ast::GraphNode,
}

// ----------------------
//  BEGIN Default nodes
// ----------------------

struct InputNode;
impl<'a, 'b, 'c> GraphNodeBuilder<InputNode> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct DefaultNode;
impl<'a, 'b, 'c> GraphNodeBuilder<DefaultNode> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        for call in self.node.calls {
            // Step 1. get the node
            let callee = self.root.get(&call.name)?;
            todo!();
        }
        todo!()
    }
}

// ----------------------
//  BEGIN Built-in nodes
// ----------------------

struct Transform;
impl<'a, 'b, 'c> GraphNodeBuilder<Transform> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct ToLinear;
impl<'a, 'b, 'c> GraphNodeBuilder<ToLinear> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct Concat;
impl<'a, 'b, 'c> GraphNodeBuilder<Concat> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

// ----------------------
//  MATCH Built-in nodes
// ----------------------

macro_rules! match_builtins(
    ($s:ident => $( $t:ty ),*,) => {
        match $s.node.calls[0].name.as_str() {
            $( stringify!($t) => GraphNodeBuilder::<$t>::build($s) ),*,
            _ => GraphNodeBuilder::<DefaultNode>::build($s),
        }
    }
);

impl<'a, 'b, 'c> GraphNodeEntry<'a, 'b, 'c> {
    pub fn build(self) -> Result<()> {
        if self.id == 0 {
            // input node
            GraphNodeBuilder::<InputNode>::build(self)
        } else {
            match_builtins!(self =>
                Transform,
                ToLinear,
                Concat,
            )
        }
    }
}

// ----------------------
//   END  Built-in nodes
// ----------------------

trait GraphNodeBuilder<B>
where
    Self: Sized,
{
    fn build(self) -> Result<()>;
}
