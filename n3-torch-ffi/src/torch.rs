use pyo3::prelude::*;

pub struct Torch<'a>(pub Python<'a>);

impl<'a> Torch<'a> {
    pub fn nn(&self, name: &str) -> PyResult<&PyAny> {
        let nn = PyModule::import(self.0, "torch.nn")?;
        let cls = nn.get(name)?;
        cls.getattr("__init__")
    }
}
