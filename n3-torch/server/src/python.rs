use n3_machine_ffi::{Machine, Program, SignalHandler, WorkHandler, WorkStatus};
use n3_torch_ffi::PyMachine;

pub struct PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    inner: T,
    handler: Option<WorkHandler>,
}

impl<T> PyMachineBase<T>
where
    T: PyMachine + 'static,
{
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            handler: None,
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
    fn spawn(&mut self, program: &mut Program, handler: &SignalHandler) -> WorkStatus {
        if self.handler.is_some() {
            return self.status();
        }

        let handler = WorkHandler::new_with_signal(&program.id, handler).unwrap();
        handler.start().unwrap();

        self.inner.py_spawn(program, &handler).unwrap();
        self.handler = Some(handler);

        self.status()
    }

    fn status(&mut self) -> WorkStatus {
        if let Some(handler) = &self.handler {
            handler.status().unwrap()
        } else {
            WorkStatus::default()
        }
    }

    fn join(&mut self) -> WorkStatus {
        self.terminate()
    }

    fn terminate(&mut self) -> WorkStatus {
        self.inner.py_terminate().unwrap();

        self.status()
    }
}
