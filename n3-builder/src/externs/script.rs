use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ast;
use crate::context::{Build, CloneSafe};
use crate::error::Result;
use crate::nodes::NodeRoot;
use crate::seed::Seed;

pub type PythonScripts = BTreeMap<String, PythonScript>;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PythonScript {
    pub name: String,
    pub source: String,
}

impl PartialEq for PythonScript {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
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
