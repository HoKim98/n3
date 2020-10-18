use std::io;

use crate::code::Codes;
use crate::compact::Program as CompactedProgram;
use crate::error::Result;
use crate::externs::PythonScripts;
use crate::graph::Variables;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub graph: Variables,
    pub nodes: Codes,
    pub scripts: PythonScripts,
}

impl Program {
    pub fn compact(&self) -> CompactedProgram {
        CompactedProgram::compact(self)
    }

    pub fn save<W>(&self, writer: W) -> Result<()>
    where
        W: io::Write,
    {
        self.compact().save(writer)
    }

    pub fn load<R>(reader: R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(CompactedProgram::load(reader)?.decompact())
    }
}
