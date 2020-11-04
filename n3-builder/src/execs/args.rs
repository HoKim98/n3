use std::ops::Deref;

use super::ir::ExecIR;
use super::program::Program;
use super::root::ExecRoot;
use super::var::Vars;
use crate::error::Result;

pub struct Args<'a> {
    pub(super) root: &'a mut ExecRoot,
    pub(super) ir: ExecIR,
    pub(super) args: Vars,
}

impl<'a> Deref for Args<'a> {
    type Target = Vars;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl<'a> Args<'a> {
    pub fn build_uncompacted(self) -> Result<Program> {
        self.ir.build(&self.root)
    }

    pub fn build_with_env(self) -> Result<Vec<u8>> {
        let mut program = self.ir.build(&self.root)?;
        self.root.attach_env(&mut program);
        program.save_to_binary().map_err(|e| e.into())
    }

    pub fn build(self) -> Result<Vec<u8>> {
        self.build_uncompacted()?
            .save_to_binary()
            .map_err(|e| e.into())
    }
}
