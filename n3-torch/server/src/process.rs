use std::process::{Child, Command};
use std::str;

use n3_machine_ffi::{LocalQuery, Machine, NetError, Program, Query, Result, WorkHandler};
use n3_torch_ffi::{ProcessMachine as ProcessMachineTrait, PyMachine};

use crate::python::PyMachineBase;

pub struct ProcessMachine {
    process: Option<Child>,
    handler: Option<WorkHandler>,
    query: Query,
}

impl ProcessMachine {
    pub unsafe fn try_new<T>(query: &Query) -> Vec<Box<dyn Machine>>
    where
        T: ProcessMachineTrait<Self> + 'static,
    {
        T::verify_query(query)
            .into_iter()
            .map(Self::new)
            .map(T::new)
            .map(PyMachineBase::new)
            .map(|x| x.into_box_trait())
            .collect()
    }

    fn new(query: Query) -> Self {
        Self {
            process: None,
            handler: None,
            query,
        }
    }
}

impl PyMachine for ProcessMachine {
    fn is_running(&self) -> bool {
        self.process.is_some()
    }

    fn py_spawn(&mut self, program: &mut Program, handler: &WorkHandler) -> Result<()> {
        self.handler = Some(handler.clone());

        // the machine's name
        program.machine = LocalQuery(&self.query).to_string();

        // store the program into a file
        program.save()?;

        // spawn to new process
        let process = Command::new("n3-torchc")
            .arg(program.id.work.to_string())
            .arg(program.id.primary.to_string())
            .spawn()
            .map_err(NetError::from)?;
        self.process = Some(process);

        Ok(())
    }

    fn py_terminate(&mut self) -> Result<()> {
        match self.process.take() {
            Some(process) => {
                let result = process.wait_with_output().map_err(NetError::from)?;
                dbg!(&result);
                if !result.status.success() {
                    let handler = self.handler.as_ref().unwrap();

                    let msg = match str::from_utf8(&result.stderr) {
                        Ok("") | Err(_) => "trap encountered",
                        Ok(msg) => msg,
                    };
                    handler.end_err(msg)?;
                }
                Ok(())
            }
            None => Ok(()),
        }
    }
}
