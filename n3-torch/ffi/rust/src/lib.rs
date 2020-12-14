use std::ops::{Deref, DerefMut};

use n3_machine_ffi::{Program, Query, Result, WorkHandler};

pub trait PyMachine {
    fn is_running(&self) -> bool;

    fn py_spawn(&mut self, program: &mut Program, handler: &WorkHandler) -> Result<()>;

    fn py_terminate(&mut self) -> Result<()>;
}

pub trait ProcessMachine<P>: PyMachine {
    fn new(process: P) -> Self
    where
        Self: Sized;

    fn verify_query(query: &Query) -> Vec<Query>;
}

impl<T, P> PyMachine for T
where
    T: ProcessMachine<P> + Deref<Target = P> + DerefMut,
    P: PyMachine,
{
    fn is_running(&self) -> bool {
        self.deref().is_running()
    }

    fn py_spawn(&mut self, program: &mut Program, handler: &WorkHandler) -> Result<()> {
        self.deref_mut().py_spawn(program, handler)
    }

    fn py_terminate(&mut self) -> Result<()> {
        self.deref_mut().py_terminate()
    }
}
