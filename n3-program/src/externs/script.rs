use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

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
