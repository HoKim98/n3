use super::code::ExternCode;
use crate::ast;
use crate::error::Result;
use crate::nodes::NodeRoot;

#[derive(Clone)]
pub struct ExternIR {
    data: ExternIRData,
    shapes: ExternIRShapes,
}

#[derive(Clone)]
pub struct ExternIRData {
    pub id: u64,
    pub name: String,
    pub kwargs: ast::Keywords,
    pub input: ast::Outs,
    pub output: ast::Outs,
}

#[derive(Clone)]
pub struct ExternIRShapes {
    input: ast::ShapesDict,
    output: ast::ShapesDict,
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
            input: ast::ShapesDict(self.input.keys().map(|x| (x.clone(), None)).collect()),
            output: ast::ShapesDict(self.output.keys().map(|x| (x.clone(), None)).collect()),
        }
    }
}

impl ExternIR {
    pub fn get_input_shapes(&self) -> Option<&ast::ShapesDict> {
        Some(&self.shapes.input)
    }

    pub fn get_output_shapes(&self) -> Option<&ast::ShapesDict> {
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
