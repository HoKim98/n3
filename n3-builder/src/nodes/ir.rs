use super::code::NodeCode;
use super::root::NodeRoot;
use crate::ast;
use crate::code::{Code, CodeData, DataFromIR};
use crate::context::{Build, CloneSafe};
use crate::error::{GraphCallError, Result};
use crate::graph::Graph;
use crate::seed::Seed;
use crate::tensor::{IRData, TensorGraph, TensorNode};
use crate::variable::{BuildValue, CloneValue, Link};

#[derive(Debug, PartialEq)]
pub struct NodeIR {
    pub data: IRData,
    pub ty: ast::LetNodeType,
    pub tensor_graph: TensorGraph,
    pub repeat: Option<ast::Value>,
}

impl NodeIR {
    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        self.tensor_graph.get_input_shapes()
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        self.tensor_graph.get_output_shapes()
    }

    pub fn build(mut self, root: &NodeRoot) -> Result<Code> {
        if let Some(repeat) = &self.repeat {
            let repeat = repeat.build();
            let repeat = repeat
                .unwrap_uint()
                .ok_or_else(|| GraphCallError::MismatchedArgType {
                    expected: ast::LetType::UInt,
                    given: repeat.ty(),
                })?;

            match repeat {
                1 => {}
                // remove nodes
                0 => {
                    // the input-output shapes should be same
                    self.get_input_shapes().link_to(&self.get_output_shapes())?;
                    // clean up graph
                    self.tensor_graph.clear();
                }
                // repeat nodes
                _ => {
                    let repeat = repeat as usize;

                    let update_out_id = |out: &mut ast::Out| {
                        for node in self.tensor_graph.iter().rev() {
                            if let Some(shapes) = node.get_output_shapes() {
                                if shapes.0.borrow().contains_key(&out.name) {
                                    out.id = Some(node.get_id());
                                    return;
                                }
                            }
                        }
                        unreachable!()
                    };

                    let mut cloned_graph = TensorGraph::empty();
                    for _ in 0..(repeat - 1) {
                        for node in self.tensor_graph.iter() {
                            let mut node = {
                                let mut variables = vec![];
                                node.clone_safe(&root.seed, &mut variables)
                            };

                            {
                                let dims = node.get_graph().borrow_mut().unload_dims();

                                // match shapes
                                let last_outputs = cloned_graph
                                    .get_output_shapes()
                                    .or_else(|| self.get_output_shapes());
                                let new_inputs = node.get_input_shapes();
                                last_outputs.link_to(&new_inputs)?;

                                // update graph node ids
                                for out in node.get_inputs_mut().values_mut() {
                                    update_out_id(out);
                                }
                                for out in node.get_outputs_mut().values_mut() {
                                    update_out_id(out);
                                }

                                node.get_graph().borrow_mut().load_dims_weakly(dims);
                            }

                            cloned_graph.push(node);
                        }
                    }
                    self.tensor_graph.append(&mut cloned_graph);
                }
            }
        }

        // extern node
        if let ast::LetNodeType::Extern(_) = self.ty {
            // drop reference count
            drop(self.data.graph);

            // repeat
            if self.tensor_graph.len() == 1 {
                let mut node = self.tensor_graph.pop().unwrap().unwrap_extern().unwrap();

                // pass the IO Outs
                node.data.input = self.data.input;
                node.data.output = self.data.output;

                return Ok(node.build()?.into());
            } else {
                // re-define graph
                self.data.graph = Graph::new(&root.seed).into();
            }
        }

        let tensor_graph = self.tensor_graph.build(root)?;

        Ok(NodeCode {
            data: CodeData::from_ir(self.data),
            tensor_graph,
        }
        .into())
    }
}

impl Build for NodeIR {
    type Output = TensorNode;

    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self::Output> {
        TensorNode::build(root, name, source)
    }
}

impl CloneSafe for NodeIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        match self.ty {
            // extern node wrapper
            // note: ordered (extern_node -> graph(copy) -> data -> repeat)
            ast::LetNodeType::Extern(_) => {
                let tensor_graph = self.tensor_graph.clone_safe(seed, variables);
                let node = tensor_graph.try_borrow_extern_node().unwrap();
                let graph = node.data.graph.clone();

                let mut data = self.data.clone();
                data.graph = graph;

                Self {
                    data,
                    ty: self.ty,
                    tensor_graph,
                    repeat: self.repeat.clone_value(variables),
                }
            }
            // note: ordered (data -> tensor_graph -> repeat)
            ast::LetNodeType::Default => Self {
                data: self.data.clone_safe(seed, variables),
                ty: self.ty,
                tensor_graph: self.tensor_graph.clone_safe(seed, variables),
                repeat: self.repeat.clone_value(variables),
            },
        }
    }
}
