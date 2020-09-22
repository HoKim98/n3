use super::code::ExternCode;
use crate::ast;
use crate::context::CloneSafe;
use crate::error::Result;
use crate::graph::RefGraph;
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::IRData;
use crate::variable::CloneValue;

#[derive(Debug)]
pub struct ExternIR {
    pub data: IRData,
    shapes: ExternIRShapes,
}

#[derive(Debug)]
pub struct ExternIRShapes {
    input: Option<ast::Shapes>,
    output: Option<ast::Shapes>,
}

impl From<IRData> for ExternIR {
    fn from(data: IRData) -> Self {
        Self {
            shapes: (&data).into(),
            data,
        }
    }
}

impl<'a> From<&'a IRData> for ExternIRShapes {
    fn from(data: &'a IRData) -> Self {
        Self {
            input: Some(ast::Shapes::new(
                data.input.keys().map(|x| (x.clone(), None)).collect(),
            )),
            output: Some(ast::Shapes::new(
                data.output.keys().map(|x| (x.clone(), None)).collect(),
            )),
        }
    }
}

impl ExternIR {
    pub fn new(
        name: String,
        graph: RefGraph,
        input: Option<ast::Shapes>,
        output: Option<ast::Shapes>,
    ) -> Self {
        Self {
            data: IRData::with_shapes(name, graph, input.as_ref(), output.as_ref()),
            shapes: ExternIRShapes { input, output },
        }
    }

    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        self.shapes.input.as_ref()
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        self.shapes.output.as_ref()
    }

    pub fn build(self, root: &NodeRoot) -> Result<ExternCode> {
        Ok(ExternCode {
            script: root.get_extern(&self.data.name)?,
            name: self.data.name,
            graph: self.data.graph,
            input: self.data.input,
            output: self.data.output,
        })
    }
}

impl CloneSafe for ExternIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        // note: ordered (data -> shapes)
        Self {
            data: self.data.clone_safe(seed, variables),
            shapes: self.shapes.clone_safe(seed, variables),
        }
    }
}

impl CloneSafe for ExternIRShapes {
    fn clone_safe(&self, _: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        Self {
            input: self.input.as_ref().map(|x| x.clone_value(variables)),
            output: self.output.as_ref().map(|x| x.clone_value(variables)),
        }
    }
}
