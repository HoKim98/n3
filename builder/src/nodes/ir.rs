use super::code::NodeCode;
use super::root::NodeRoot;
use crate::ast;
use crate::cache::{Build, CloneSafe};
use crate::error::Result;
use crate::graph::Graph;
use crate::seed::Seed;
use crate::tensor::TensorGraph;

pub struct NodeIR {
    pub name: String,
    pub graph: Graph,
    pub tensor_graph: TensorGraph,
    pub data: NodeIRData,
}

#[derive(Default)]
pub struct NodeIRData {
    pub id: u64,
    pub input: Option<ast::Outs>,
    pub output: Option<ast::Outs>,
    pub repeat: Option<ast::Value>,
}

impl NodeIR {
    pub fn get_input_shapes(&self) -> Option<&ast::ShapesDict> {
        self.tensor_graph.get_input_shapes()
    }

    pub fn get_output_shapes(&self) -> Option<&ast::ShapesDict> {
        self.tensor_graph.get_output_shapes()
    }

    pub fn build(self, root: &NodeRoot) -> Result<NodeCode> {
        let input = unwrap_outs(0, &self.tensor_graph, self.data.input, |g| {
            g.get_input_shapes().unwrap()
        });
        let output = unwrap_outs(1, &self.tensor_graph, self.data.output, |g| {
            g.get_output_shapes().unwrap()
        });

        if let Some(repeat) = self.data.repeat {
            todo!()
        }

        let tensor_graph = self.tensor_graph.build(root)?;

        Ok(NodeCode {
            name: self.name,
            input,
            output,
            graph: tensor_graph,
        })
    }
}

impl CloneSafe for NodeIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        todo!()
    }
}

impl Build for NodeIR {
    fn build(root: &NodeRoot, name: &str, source: String) -> Result<Self> {
        todo!()
    }
}

fn unwrap_outs<'a, F>(
    id: u64,
    graph: &'a TensorGraph,
    outs: Option<ast::Outs>,
    fn_get_shapes: F,
) -> ast::Outs
where
    F: FnOnce(&'a TensorGraph) -> &'a ast::ShapesDict,
{
    match outs {
        Some(input) => input,
        None => fn_get_shapes(graph)
            .0
            .keys()
            .map(|x| {
                (
                    x.clone(),
                    ast::Out {
                        id: Some(id),
                        name: Some(x.clone()),
                    },
                )
            })
            .collect(),
    }
}
