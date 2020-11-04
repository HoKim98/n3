use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

use n3_program::ToValues;
use n3_torch_ffi::pyo3;

use super::value::TryToPyObject;

#[derive(Debug)]
pub struct Args<'a, T>(pub &'a T)
where
    T: ToValues;

impl<'a, T> IntoPyDict for Args<'a, T>
where
    T: ToValues,
{
    fn into_py_dict(self, py: Python) -> &PyDict {
        self.0
            .to_values()
            .into_iter()
            .map(|(k, v)| (k, v.try_to_object(py)))
            .into_py_dict(py)
    }
}
