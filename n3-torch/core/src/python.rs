use pyo3::{PyErr, PyObject, Python};

use chrono::prelude::*;

use n3_machine_ffi::{Machine, MachineIdSet, Program, SignalHandler, WorkStatus};
use n3_torch_ffi::{pyo3, PyMachine};

pub struct PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    inner: T,
    kwargs: Option<PyObject>,
    cache: WorkStatus,
}

impl<T> PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            kwargs: None,
            cache: Default::default(),
        }
    }

    pub fn into_box_trait(self) -> Box<dyn Machine> {
        Box::new(self)
    }
}

impl<T> PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    fn store_py_error(&mut self, e: PyErr) {
        self.cache.is_running = false;
        self.cache.error_msg = Python::with_gil(|py| {
            e.print_and_set_sys_last_vars(py);
            Some(e.to_string())
        });
    }
}

impl<T> Machine for PyMachineBase<T>
where
    T: PyMachine,
{
    fn spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
        _: SignalHandler,
    ) -> WorkStatus {
        if self.cache.error_msg.is_some() {
            return self.cache.clone();
        }

        match self.inner.py_spawn(id, program, command) {
            Ok(kwargs) => {
                self.kwargs = Some(kwargs);
                self.cache.is_running = true;
            }
            Err(e) => self.store_py_error(e),
        }
        self.cache.clone()
    }

    fn status(&mut self) -> WorkStatus {
        Python::with_gil(|py| {
            let kwargs = self.kwargs.as_ref().unwrap().as_ref(py);
            self.cache.is_running = kwargs.get_item("is_running")?.extract()?;
            self.cache.error_msg = kwargs.get_item("error_msg")?.extract()?;
            let date_begin: Option<_> = kwargs.get_item("date_begin")?.extract()?;
            let date_end: Option<_> = kwargs.get_item("date_end")?.extract()?;

            self.cache.date_begin = date_begin.map(|(secs, nsecs)| {
                DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
            });
            self.cache.date_end = date_end.map(|(secs, nsecs)| {
                DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
            });
            Ok(())
        })
        .unwrap_or_else(|e| self.store_py_error(e));

        self.cache.clone()
    }

    fn join(&mut self) -> WorkStatus {
        self.terminate()
    }

    fn terminate(&mut self) -> WorkStatus {
        self.inner
            .py_terminate()
            .unwrap_or_else(|e| self.store_py_error(e));

        self.status()
    }
}
