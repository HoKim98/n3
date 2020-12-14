use std::collections::BTreeMap;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

use n3_program::{ast, BuildValue};

pub trait TryToPyObject {
    fn try_to_object(&self, py: Python) -> Option<PyObject>;
}

impl TryToPyObject for ast::RefVariable {
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        self.borrow().value.try_to_object(py)
    }
}

impl TryToPyObject for ast::Expr {
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        self.build().try_to_object(py)
    }
}

impl TryToPyObject for ast::Value {
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        match self {
            Self::Bool(v) => Primitive(v.to_object(py)).try_to_object(py),
            Self::UInt(v) => Primitive(v.to_object(py)).try_to_object(py),
            Self::Int(v) => Primitive(v.to_object(py)).try_to_object(py),
            Self::Real(v) => Primitive(v.to_object(py)).try_to_object(py),
            Self::String(v) => Primitive(v.to_object(py)).try_to_object(py),
            Self::Variable(v) => v.try_to_object(py),
            Self::Expr(v) => v.try_to_object(py),
            Self::List(v) => v.try_to_object(py),
            Self::Map(v) => v.try_to_object(py),
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

impl<T> TryToPyObject for Box<T>
where
    T: TryToPyObject,
{
    fn try_to_object(&self, py: Python) -> Option<PyObject> {
        (**self).try_to_object(py)
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
