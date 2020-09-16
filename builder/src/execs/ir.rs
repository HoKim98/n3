use super::program::Program;
use crate::ast;
use crate::context::CloneSafe;
use crate::error::Result;
use crate::nodes::NodeRoot;
use crate::seed::Seed;
use crate::tensor::IRData;

#[derive(Debug)]
pub struct ExecIR {
    pub data: IRData,
}

impl ExecIR {
    pub fn build(self, root: &NodeRoot) -> Result<Program> {
        todo!()
    }
}

impl CloneSafe for ExecIR {
    fn clone_safe(&self, seed: &Seed, variables: &mut Vec<ast::RefVariable>) -> Self {
        todo!()
    }
}
