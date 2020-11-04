use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ctrlc::{set_handler, Error};
use pyo3::prelude::*;

use n3_torch_ffi::pyo3;

#[pyclass]
#[derive(Clone)]
pub struct SignalHandler {
    running: Arc<AtomicBool>,
}

#[pymethods]
impl SignalHandler {
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl SignalHandler {
    fn try_new() -> Result<Self, Error> {
        let running = Arc::new(AtomicBool::new(true));
        let running_receiver = running.clone();

        set_handler(move || running_receiver.store(false, Ordering::SeqCst))?;

        Ok(SignalHandler { running })
    }

    pub fn run<R>(py: Python, f: impl Fn(&PyCell<Self>) -> R) -> Result<R, Error> {
        let handler = Self::try_new()?;
        let handler = PyCell::new(py, handler).unwrap();

        Ok(f(handler))
    }
}
