use pyo3::prelude::*;

use n3_torch_ffi::pyo3;

/// note: https://stackoverflow.com/questions/44445593/how-to-ignore-python-modules-during-import
#[pyclass]
pub struct ImportBlocker {
    pub module: Py<PyAny>,
}

#[pymethods]
impl ImportBlocker {
    fn find_module(
        self_: PyRef<Self>,
        py: Python,
        fullname: &str,
        _path: Option<&str>,
    ) -> PyResult<Option<Py<PyAny>>> {
        if fullname == "n3_torch_ffi" {
            Ok(Some(self_.into_py(py)))
        } else {
            Ok(None)
        }
    }

    fn create_module(&self, py: Python, _spec: Py<PyAny>) -> PyResult<Py<PyAny>> {
        Ok(self.module.clone().into_py(py))
    }

    fn exec_module(&self, _mdl: Py<PyAny>) {}
}
