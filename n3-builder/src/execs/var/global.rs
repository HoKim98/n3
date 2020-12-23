use std::ops::Deref;
use std::path::PathBuf;

use lazy_static::lazy_static;

use super::super::dirs::*;
use super::core::Query;
use super::env::EnvVars;
use crate::ast;
use crate::graph::{ToValues, Values};

#[derive(Clone, Debug)]
pub struct GlobalVars {
    inner: EnvVars,
}

lazy_static! {
    static ref QUERY: Vec<Query<'static>> = vec![
        Query {
            name: N3_SOURCE_ROOT,
            description: "The n3 standard library's path",
            ty: ast::LetType::String,
            value: None,
            fn_value: None,
        },
        Query {
            name: N3_ROOT,
            description: "The n3 local path",
            ty: ast::LetType::String,
            value: None,
            fn_value: Some(GlobalVars::default_home_dir),
        },
        Query {
            name: N3_MACHINES,
            description: "The n3 machines",
            ty: ast::LetType::List(Box::new(ast::LetType::String)),
            value: None,
            fn_value: None,
        },
    ];
}

impl Default for GlobalVars {
    fn default() -> Self {
        Self {
            inner: EnvVars::load(QUERY.to_vec()).unwrap(),
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

    pub(crate) fn default_home_dir() -> Option<String> {
        dirs::home_dir()
            .map(|mut dir| {
                dir.push(".n3");
                dir
            })
            .map(|x| x.display().to_string())
    }

    #[cfg(feature = "pip")]
    pub(crate) fn get_n3_source_root() -> PathBuf {
        let output = std::process::Command::new("python")
            .arg("-c")
            .arg("import n3; print(n3.__file__)")
            .output()
            .expect("failed to execute python");

        if !output.stderr.is_empty() {
            panic!("variable 'N3_SOURCE_ROOT' is incorrect");
        }

        let path = String::from_utf8(output.stdout).unwrap();
        let mut path = PathBuf::from(path.trim_end());
        path.pop();
        path
    }

    #[cfg(not(feature = "pip"))]
    pub(crate) fn get_n3_source_root() -> PathBuf {
        panic!(
            "variable 'N3_SOURCE_ROOT' is not given. Please set feature pip=true to search automatically."
        )
    }
}

impl ToValues for GlobalVars {
    fn to_values(&self) -> Values {
        self.inner.to_values()
    }
}
