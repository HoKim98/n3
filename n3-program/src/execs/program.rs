use std::io;

use crate::code::Codes;
use crate::compact::Program as CompactedProgram;
use crate::error::Result;
use crate::externs::PythonScripts;
use crate::graph::{Values, Variables};

pub const PROGRAM_MAIN: &str = "__main__";

#[derive(Debug)]
pub struct Program {
    pub env: Option<Values>,
    pub graph: Variables,
    pub nodes: Codes,
    pub scripts: PythonScripts,
}

impl PartialEq for Program {
    fn eq(&self, other: &Self) -> bool {
        // ignore env
        self.graph.eq(&other.graph)
            && self.nodes.eq(&other.nodes)
            && self.scripts.eq(&other.scripts)
    }
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

    pub fn save_to_binary(&self) -> Result<Vec<u8>> {
        let mut binary = vec![];
        self.save(&mut binary)?;
        Ok(binary)
    }

    pub fn load<R>(reader: R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(CompactedProgram::load(reader)?.decompact())
    }
}
