use pyo3::{PyResult, Python};
use pyo3_mp::Process;

use n3_machine::{Machine, MachineId, Program, Query};
use n3_torch_ffi::{pyo3, ProcessMachine as ProcessMachineTrait, PyMachine};

use crate::exec::n3_execute_wrapper;
use crate::python::PyMachineBase;

pub struct ProcessMachine {
    process: Process<'static>,
    query: Query,
}

impl ProcessMachine {
    pub unsafe fn try_new<T>(query: &Query) -> Vec<Box<dyn Machine>>
    where
        T: ProcessMachineTrait<Self> + 'static,
    {
        T::verify_query(query)
            .into_iter()
            .map(|x| ProcessMachine::_new(x))
            .flatten()
            .map(|x| T::try_new(x))
            .map(PyMachineBase)
            .map(|x| x.into_box_trait())
            .collect()
    }

    unsafe fn _new(query: Query) -> Option<Self> {
        let py = Python::assume_gil_acquired();

        Some(Self {
            process: Process::new(py).ok()?,
            query,
        })
    }
}

impl PyMachine for ProcessMachine {
    fn is_running(&self) -> bool {
        self.process.is_running()
    }

    fn py_spawn(&mut self, id: MachineId, program: &Program, command: &str) -> PyResult<()> {
        // the GIL is acquired by HostMachine
        let py = unsafe { Python::assume_gil_acquired() };

        // the machine's name
        let machine = format!("{}", self.query);

        // the function to execute the program
        let n3_execute = n3_execute_wrapper(py)?;

        // spawn to new process
        self.process
            .spawn(n3_execute, (id, &machine, command, program), None)?;
        Ok(())
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.process.join()
    }
}
