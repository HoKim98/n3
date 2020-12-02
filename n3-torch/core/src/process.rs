use pyo3::types::IntoPyDict;
use pyo3::{IntoPy, PyObject, PyResult, Python};
use pyo3_mp::Process;

use chrono::prelude::*;

use n3_machine_ffi::{LocalQuery, Machine, MachineIdSet, Program, Query};
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
            .map(PyMachineBase::new)
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

    fn py_spawn(
        &mut self,
        id: MachineIdSet,
        program: &Program,
        command: &str,
    ) -> PyResult<PyObject> {
        // the GIL is acquired by HostMachine
        let py = unsafe { Python::assume_gil_acquired() };

        // the machine's name
        let machine = format!("{}", LocalQuery(&self.query));

        // the function to execute the program
        let n3_execute = n3_execute_wrapper(py)?;

        // the arguments passed to the program
        let kwargs = [
            ("work_id", id.work.into_py(py)),
            ("is_running", true.into_py(py)),
            (
                "date_begin",
                Some((Utc::now().timestamp(), Utc::now().nanosecond())).into_py(py),
            ),
            ("date_end", None::<(i64, u32)>.into_py(py)),
        ]
        .into_py_dict(py);

        // spawn to new process
        let (_, kwargs) = self.process.spawn_mut(
            n3_execute,
            (id.primary, id.local, id.world, &machine, command, program),
            Some(kwargs),
        )?;

        Ok(kwargs)
    }

    fn py_terminate(&mut self) -> PyResult<()> {
        self.process.join()
    }
}
