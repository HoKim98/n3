use super::node::NodeEntry;
use crate::ast;
use crate::error::{GraphCallError, Result};

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
        let id = self.id;

        for call in self.node.calls {
            // Step 1. get the node
            let mut callee = self.root.get(&call.name)?;
            let graph = self.root.graph.borrow();

            callee.set_id(id);
            callee.set_repeat(graph.replace_to(call.repeat)?);

            // Step 2. apply variables
            if let Some(args) = call.args {
                let args = args
                    .into_iter()
                    .map(|(k, v)| {
                        let v = graph.replace_to(Some(v))?;
                        let var = ast::Variable::with_name_value(k.clone(), v);
                        Ok((k, var.into()))
                    })
                    .collect::<Result<_>>()?;
                callee.apply_variables(args)?;
            }

            // Step 3. apply IO
            let inputs = unwrap_dict(call.inputs.unwrap_or_default())?;
            let callee_inputs = callee.get_inputs();

            todo!()
        }
        todo!()
    }
}

fn unwrap_dict(inputs: ast::GraphInputs) -> Result<ast::Outs> {
    let given = inputs.ty();
    inputs.unwrap_dict().ok_or_else(|| {
        GraphCallError::MismatchedInputs {
            expected: ast::GraphInputsType::Dict,
            given,
        }
        .into()
    })
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
    fn is_input(&self) -> bool {
        self.id == 0
    }

    pub fn build(self) -> Result<()> {
        if self.is_input() {
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
