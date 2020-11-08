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
        Some(dirs::home_dir().unwrap().display().to_string())
    }

    #[cfg(feature = "pip")]
    pub(crate) fn get_n3_source_root() -> PathBuf {
        use n3_torch_ffi::pyo3::prelude::*;
        Python::with_gil(|py| {
            py.run("import n3", None, None)
                .and_then(|()| py.eval("n3.__file__", None, None))
                .map(|x| x.str().unwrap())
                .map(|x| x.to_string())
                .map(PathBuf::from)
                .map(|mut x| {
                    x.pop(); // remove __init__.py
                    x
                })
                .map_err(|e| {
                    e.print_and_set_sys_last_vars(py);
                })
                .expect("variable 'N3_SOURCE_ROOT' is incorrect")
        })
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
