use super::node::NodeEntry;
use crate::ast;
use crate::error::Result;

trait GraphNodeBuilder<B>
where
    Self: Sized,
{
    fn build(self) -> Result<()>;
}

pub struct GraphNodeEntry<'a, 'b> {
    pub root: &'b mut NodeEntry<'a>,
    pub id: u64,
    pub node: ast::GraphNode,
}

struct InputNode;
impl<'a, 'b> GraphNodeBuilder<InputNode> for GraphNodeEntry<'a, 'b> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct DefaultNode;
impl<'a, 'b> GraphNodeBuilder<DefaultNode> for GraphNodeEntry<'a, 'b> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

// ----------------------
//  BEGIN Built-in nodes
// ----------------------

struct Transform;
impl<'a, 'b> GraphNodeBuilder<Transform> for GraphNodeEntry<'a, 'b> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct ToLinear;
impl<'a, 'b> GraphNodeBuilder<ToLinear> for GraphNodeEntry<'a, 'b> {
    fn build(self) -> Result<()> {
        todo!()
    }
}

struct Concat;
impl<'a, 'b> GraphNodeBuilder<Concat> for GraphNodeEntry<'a, 'b> {
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

impl<'a, 'b> GraphNodeEntry<'a, 'b> {
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
