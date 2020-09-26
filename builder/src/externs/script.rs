use serde::{Deserialize, Serialize};

use crate::ast;
use crate::context::{Build, CloneSafe};
use crate::error::Result;
use crate::nodes::NodeRoot;
use crate::seed::Seed;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PythonScript {
    name: String,
    source: String,
}

impl CloneSafe for PythonScript {
    fn clone_safe(&self, _seed: &Seed, _variables: &mut Vec<ast::RefVariable>) -> Self {
        self.clone()
    }
}

impl Build for PythonScript {
    type Output = Self;

    fn build(_root: &NodeRoot, name: &str, source: String) -> Result<Self::Output>
    where
        Self: Sized,
    {
        Ok(PythonScript {
            name: name.to_string(),
            source,
        })
    }
}
