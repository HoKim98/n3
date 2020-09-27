use std::ops::Deref;
use std::path::PathBuf;

use super::core::Query;
use super::env::EnvVars;
use crate::ast;

pub struct GlobalVars {
    inner: EnvVars,
}

const QUERY: &[Query] = &[Query {
    name: "root",
    ty: ast::LetType::String,
    value: None,
    fn_value: Some(GlobalVars::default_home_dir),
}];

impl Default for GlobalVars {
    fn default() -> Self {
        Self {
            inner: EnvVars::load(QUERY.to_vec()),
        }
    }
}

impl Deref for GlobalVars {
    type Target = EnvVars;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl GlobalVars {
    pub fn root_dir(&self) -> PathBuf {
        PathBuf::from(self.get_string("root").unwrap())
    }

    fn default_home_dir() -> Option<String> {
        Some(dirs::home_dir().unwrap().display().to_string())
    }
}
