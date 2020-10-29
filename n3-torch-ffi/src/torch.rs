use pyo3::prelude::*;

pub struct Torch<'a>(pub Python<'a>);

impl<'a> Torch<'a> {
    pub fn this(&self, name: &str) -> PyResult<&PyAny> {
        self.get("torch", name)
    }

    pub fn nn(&self, name: &str) -> PyResult<&PyAny> {
        self.get("torch.nn", name)
    }

    fn get(&self, module: &'static str, name: &str) -> PyResult<&PyAny> {
        PyModule::import(self.0, module)?.get(name)
    }

    pub fn terminate(&self) -> PyResult<()> {
        self.0.eval("exit(0)", None, None)?;
        Ok(())
    }
}
