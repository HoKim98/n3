use super::node::{ExternTensorGraphCondition, NodeEntry};
use crate::ast;
use crate::error::{GraphCallError, Result};
use crate::externs::ExternIR;
use crate::graph::Graph;
use crate::variable::Link;

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
        const IR_NAME: &str = "AssertShape";

        let node = self.node;

        ExternTensorGraphCondition {
            nodes: &[&node].iter().map(|&x| (x.id, x.clone())).collect(),
            names: &["Input"],
            ty_inputs: Some(ast::GraphInputsType::UseLast),
            args: Some(&[]),
            is_sized: Some(true),
            repeatable: Some(false),
            is_id_zero: true,
        }
        .test()?;

        let ir = ExternIR::new(
            IR_NAME.to_string(),
            Graph::new(self.root.ctx.root.seed.generate()).into(),
            None,
            node.shapes,
        );
        self.root.tensor_graph.push(ir.into());
        Ok(())
    }
}

struct DefaultNode;
impl<'a, 'b, 'c> GraphNodeBuilder<DefaultNode> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        let root = self.root;
        let id = self.id;

        for call in self.node.calls {
            // Step 1. get the node
            let mut callee = root.get(&call.name)?;
            let graph = root.graph.borrow();

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
                callee.apply_variables(args, true)?;
            }

            // Step 3. apply IO
            let expected_inputs = callee.get_inputs();
            let given_inputs = unwrap_dict(call.inputs.unwrap_or_default())?;
            *callee.get_inputs_mut() = expected_inputs
                .keys()
                .map(|k| match given_inputs.get(k) {
                    Some(x) => x.clone(),
                    None => ast::Out::with_name(k.clone()),
                })
                .map(|x| (x.name.clone(), x))
                .collect();

            let expected_outputs = callee.get_inputs();
            *callee.get_outputs_mut() = expected_outputs
                .keys()
                .map(|k| ast::Out::new(id, k.clone()))
                .map(|x| (x.name.clone(), x))
                .collect();

            // Step 4. merge shapes
            if root.tensor_graph.is_some() {
                let last_outputs = ast::Shapes::new(
                    callee
                        .get_inputs_mut()
                        .iter_mut()
                        .map(|(k, x)| Ok((k.clone(), root.fetch_shape(x)?)))
                        .collect::<Result<_>>()?,
                );
                let new_inputs = callee.get_input_shapes();

                if let Some(new_inputs) = new_inputs {
                    last_outputs.link_to(new_inputs)?;

                    // identity
                    let new_inputs_borrowed = new_inputs.0.borrow();
                    if let Some(new_outputs) = callee.get_output_shapes() {
                        let mut new_outputs_borrowed = new_outputs.0.borrow_mut();
                        for (name, out) in new_outputs_borrowed.iter_mut() {
                            if out.is_none() {
                                *out = new_inputs_borrowed[name].clone();
                            }
                        }
                    }
                } else {
                    for x in callee.get_inputs_mut().values_mut() {
                        x.id = Some(0);
                    }
                }
            }

            // Step 5. store
            root.tensor_graph.push(callee.into());
        }

        // Step 6. merge dedicated shapes
        if let Some(shapes) = self.node.shapes {
            if let Some(last_outputs) = root.get_output_shapes() {
                shapes.link_to(last_outputs)?;
            }
        }

        // Step 7. store id
        root.last_tensor_id = id;
        Ok(())
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

fn build_transform(
    entry: GraphNodeEntry,
    names: &'static [&'static str],
    linear: bool,
) -> Result<()> {
    let root = entry.root;
    let node = entry.node;

    ExternTensorGraphCondition {
        nodes: &[&node].iter().map(|&x| (x.id, x.clone())).collect(),
        names,
        ty_inputs: Some(ast::GraphInputsType::UseLast),
        args: Some(&[]),
        is_sized: Some(!linear),
        repeatable: Some(false),
        is_id_zero: false,
    }
    .test()?;

    // Step 1. get the IO
    let inputs = match root.get_output_shapes() {
        Some(inputs) => inputs,
        None => return GraphCallError::GenericShapes.into(),
    };
    let outputs = if linear {
        ast::Shapes::new(
            inputs
                .0
                .borrow()
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.as_ref().map(|x| ast::Shape(vec![x.0.iter().product()])),
                    )
                })
                .collect(),
        )
    } else {
        node.shapes.unwrap()
    };

    // Step 2. match the tuple
    todo!()
}

struct Transform;
impl<'a, 'b, 'c> GraphNodeBuilder<Transform> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        build_transform(self, &["Transform"], false)
    }
}

struct ToLinear;
impl<'a, 'b, 'c> GraphNodeBuilder<ToLinear> for GraphNodeEntry<'a, 'b, 'c> {
    fn build(self) -> Result<()> {
        build_transform(self, &["ToLinear"], true)
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
