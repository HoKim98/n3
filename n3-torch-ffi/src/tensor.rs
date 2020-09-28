use std::ops::Deref;

use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyclass]
pub struct TensorInput {
    inner: Py<PyDict>,
}

impl Deref for TensorInput {
    type Target = Py<PyDict>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl TensorInput {
    pub fn new(inner: Py<PyDict>) -> Self {
        Self { inner }
    }

    pub fn to_output(&self, py: Python) -> PyResult<TensorOutput> {
        let inner = PyDict::new(py);

        for (k, v) in self.inner.as_ref(py).into_iter() {
            let out = Out {
                id: Some(0),
                name: k.extract()?,
            };
            inner.set_item(out.into_py(py), v)?;
        }

        Ok(TensorOutput::new(inner.into_py(py)))
    }
}

#[pyclass]
pub struct TensorOutput {
    inner: Py<PyDict>,
}

impl TensorOutput {
    pub fn new(inner: Py<PyDict>) -> Self {
        Self { inner }
    }
}

impl Deref for TensorOutput {
    type Target = Py<PyDict>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[pyclass]
pub struct Tensor {
    pub inner: PyObject,
}

impl Deref for Tensor {
    type Target = PyObject;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<PyObject> for Tensor {
    fn from(inner: PyObject) -> Self {
        Self { inner }
    }
}

#[pyclass]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Out {
    pub id: Option<u64>,
    pub name: String,
}
