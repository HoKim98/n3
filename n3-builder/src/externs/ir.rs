use super::code::ExternCode;
use crate::ast;
use crate::code::CodeData;
use crate::context::CloneSafe;
use crate::error::Result;
use crate::graph::RefGraph;
use crate::seed::Seed;
use crate::tensor::IRData;
use crate::variable::CloneValue;

#[derive(Debug, PartialEq)]
pub struct ExternIR {
    pub ty: ast::ExternNodeType,
    pub data: IRData,
    pub shapes: ExternIRShapes,
}

#[derive(Debug, PartialEq)]
pub struct ExternIRShapes {
    pub input: Option<ast::Shapes>,
    pub output: Option<ast::Shapes>,
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
    pub fn new_first(
        ty: ast::ExternNodeType,
        name: String,
        graph: RefGraph,
        input: Option<ast::Shapes>,
        output: Option<ast::Shapes>,
    ) -> Self {
        Self {
            ty,
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

    pub fn build(self) -> Result<ExternCode> {
        Ok(ExternCode {
            ty: self.ty,
            data: CodeData::from_ir(self.data),
        })
    }
}

impl CloneSafe for ExternIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        // note: ordered (data -> shapes)
        Self {
            ty: self.ty,
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
