use std::ops::Deref;

use pyo3::prelude::*;

use crate::pyo3;

#[pyclass]
#[derive(Clone, Default)]
pub struct SignalHandler {
    inner: n3_machine_ffi::SignalHandler,
}

impl From<n3_machine_ffi::SignalHandler> for SignalHandler {
    fn from(inner: n3_machine_ffi::SignalHandler) -> Self {
        Self { inner }
    }
}

impl Deref for SignalHandler {
    type Target = n3_machine_ffi::SignalHandler;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[pymethods]
impl SignalHandler {
    pub fn is_running(&self) -> bool {
        self.inner.is_running()
    }
}

impl SignalHandler {
    pub fn run<R>(self, py: Python, f: impl Fn(&PyCell<Self>) -> R) -> R {
        let handler = PyCell::new(py, self).unwrap();
        f(handler)
    }
}
