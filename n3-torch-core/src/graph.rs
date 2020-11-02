use std::collections::BTreeMap;

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict};

use n3_builder::{ToValues, Value};
use n3_torch_ffi::pyo3;

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

trait TryToPyObject {
    fn try_to_object(&self, py: Python) -> Option<PyObject>;
}

impl TryToPyObject for Value {
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        match self {
            Value::Bool(v) => Primitive(v.to_object(py)).try_to_object(py),
            Value::UInt(v) => Primitive(v.to_object(py)).try_to_object(py),
            Value::Int(v) => Primitive(v.to_object(py)).try_to_object(py),
            Value::Real(v) => Primitive(v.to_object(py)).try_to_object(py),
            Value::String(v) => Primitive(v.to_object(py)).try_to_object(py),
            Value::List(v) => v.try_to_object(py),
            Value::Map(v) => v.try_to_object(py),
            _ => None,
        }
    }
}

impl<K, V> TryToPyObject for BTreeMap<K, V>
where
    K: ToPyObject + Clone,
    V: TryToPyObject,
{
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        let value = self
            .iter()
            .map(|(k, v)| (k.clone(), v.try_to_object(py)))
            .into_py_dict(py);
        Primitive(value).try_to_object(py)
    }
}

impl<T> TryToPyObject for Vec<T>
where
    T: TryToPyObject,
{
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        let value = pyo3::types::PyList::new(py, self.iter().map(|x| x.try_to_object(py)));
        Primitive(value).try_to_object(py)
    }
}

impl<T> TryToPyObject for Option<T>
where
    T: TryToPyObject,
{
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        self.as_ref().and_then(|x| x.try_to_object(py))
    }
}

struct Primitive<T>(T);

impl<T> TryToPyObject for Primitive<T>
where
    T: ToPyObject,
{
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        Some(self.0.to_object(py))
    }
}
