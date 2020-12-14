use pyo3::prelude::*;

use n3_machine_ffi::WorkHandler;

#[pyclass]
pub struct PyHandler {
    inner: WorkHandler,
}

impl PyHandler {
    pub fn new(inner: &WorkHandler) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[pymethods]
impl PyHandler {
    pub fn is_running(&self) -> bool {
        self.inner.is_running().unwrap()
    }

    pub fn update_time(&self, total_secs: i64) {
        self.inner.update_time(total_secs).unwrap()
    }

    pub fn end_ok(&self) {
        self.inner.end_ok().unwrap()
    }
}
