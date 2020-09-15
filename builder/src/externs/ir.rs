use super::code::ExternCode;
use crate::ast;
use crate::context::CloneSafe;
use crate::error::Result;
use crate::graph::RefGraph;
use crate::nodes::NodeRoot;
use crate::seed::Seed;

#[derive(Clone, Debug)]
pub struct ExternIR {
    pub data: ExternIRData,
    shapes: ExternIRShapes,
}

#[derive(Clone, Debug)]
pub struct ExternIRData {
    pub id: u64,
    pub name: String,
    pub graph: RefGraph,
    pub input: ast::Outs,
    pub output: ast::Outs,
}

#[derive(Clone, Debug)]
pub struct ExternIRShapes {
    input: ast::Shapes,
    output: ast::Shapes,
}

impl Into<ExternIR> for ExternIRData {
    fn into(self) -> ExternIR {
        ExternIR {
            shapes: (&self).into(),
            data: self,
        }
    }
}

impl<'a> Into<ExternIRShapes> for &'a ExternIRData {
    fn into(self) -> ExternIRShapes {
        ExternIRShapes {
            input: ast::Shapes(self.input.keys().map(|x| (x.clone(), None)).collect()),
            output: ast::Shapes(self.output.keys().map(|x| (x.clone(), None)).collect()),
        }
    }
}

impl ExternIR {
    pub fn get_input_shapes(&self) -> Option<&ast::Shapes> {
        Some(&self.shapes.input)
    }

    pub fn get_output_shapes(&self) -> Option<&ast::Shapes> {
        Some(&self.shapes.output)
    }

    pub fn build(self, root: &NodeRoot) -> Result<ExternCode> {
        Ok(ExternCode {
            script: root.get_extern(&self.data.name)?,
            name: self.data.name,
            input: self.data.input,
            output: self.data.output,
        })
    }
}

impl CloneSafe for ExternIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        todo!()
    }
}
