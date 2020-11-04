use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

use n3_builder::ast;
use n3_torch_ffi::pyo3;

use super::value::TryToPyObject;

#[derive(Debug)]
pub struct Outs<'a>(pub &'a ast::Outs);

#[derive(Debug)]
pub struct OutsExtern<'a>(pub &'a ast::Outs);

impl<'a> IntoPyDict for Outs<'a> {
    fn into_py_dict(self, py: Python) -> &PyDict {
        self.0
            .iter()
            .map(|(k, v)| (k, v.try_to_object(py)))
            .into_py_dict(py)
    }
}

impl<'a> IntoPyDict for OutsExtern<'a> {
    fn into_py_dict(self, py: Python) -> &PyDict {
        self.0
            .iter()
            .map(|(k, v)| (k, ast::Out::new(1, v.name.clone())))
            .map(|(k, v)| (k, v.try_to_object(py)))
            .into_py_dict(py)
    }
}

impl TryToPyObject for ast::Out {
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        py.import("n3")
            .and_then(|x| x.get("util"))
            .and_then(|x| x.getattr("out"))
            .and_then(|x| x.call_method1("Out", (self.id, &self.name)))
            .map(|x| x.into_py(py))
            .ok()
    }
}
