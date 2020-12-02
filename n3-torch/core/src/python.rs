use pyo3::{PyErr, PyObject, Python};

use chrono::prelude::*;

use n3_machine_ffi::{Error, Machine, MachineIdSet, Program, Result, SignalHandler, WorkStatus};
use n3_torch_ffi::{pyo3, PyMachine};

pub struct PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    inner: T,
    kwargs: Option<PyObject>,
}

impl<T> PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            kwargs: None,
        }
    }

    pub fn into_box_trait(self) -> Box<dyn Machine> {
        Box::new(self)
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
    ) -> Result<()> {
        let kwargs = self
            .inner
            .py_spawn(id, program, command)
            .map_err(|e| {
                Python::with_gil(|py| {
                    e.print_and_set_sys_last_vars(py);
                    e
                })
            })
            .map_err(|x| x.into())
            .map_err(Error::ExternalError)?;

        self.kwargs = Some(kwargs);
        Ok(())
    }

    fn status(&mut self) -> Result<WorkStatus> {
        Python::with_gil(|py| {
            let kwargs = self.kwargs.as_ref().unwrap().as_ref(py);
            let id = kwargs.get_item("work_id")?.extract()?;
            let is_running = kwargs.get_item("is_running")?.extract()?;
            let date_begin: Option<_> = kwargs.get_item("date_begin")?.extract()?;
            let date_end: Option<_> = kwargs.get_item("date_end")?.extract()?;

            let date_begin = date_begin.map(|(secs, nsecs)| {
                DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
            });
            let date_end = date_end.map(|(secs, nsecs)| {
                DateTime::from_utc(NaiveDateTime::from_timestamp(secs, nsecs), Utc)
            });

            Ok(WorkStatus {
                id,
                is_running,
                date_begin,
                date_end,
            })
        })
        .map_err(|e: PyErr| {
            Python::with_gil(|py| {
                e.print_and_set_sys_last_vars(py);
                e
            })
        })
        .map_err(|x| x.into())
        .map_err(Error::ExternalError)
    }

    fn join(&mut self) -> Result<WorkStatus> {
        self.terminate()
    }

    fn terminate(&mut self) -> Result<WorkStatus> {
        self.inner
            .py_terminate()
            .map_err(|x| x.into())
            .map_err(Error::ExternalError)?;

        self.status()
    }
}
