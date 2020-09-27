pub use crate::builder::Out;

use pyo3::PyObject;

pub trait NodeExecutable {
    fn forward(&self, kwargs: impl IntoIterator<Item = (String, Tensor)>) -> TensorOutput;
}

pub type TensorOutput = std::collections::BTreeMap<String, Tensor>;

pub struct Tensor(PyObject);

impl From<PyObject> for Tensor {
    fn from(tensor: PyObject) -> Self {
        Self(tensor)
    }
}
