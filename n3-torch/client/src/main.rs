mod args;
mod code;
mod exec;
mod handler;

use n3_machine_ffi::{Program, WorkHandler};

use self::exec::{finalize_python, n3_execute};
use self::handler::PyHandler;

fn main() {
    // load the arguments
    let id = self::args::parse_ids().expect("usage: n3-torchc id_work id_primary");

    // load the program & handler
    let program = Program::load(&id).unwrap();
    drop(id);

    let handler = WorkHandler::new(&program.id).unwrap();

    // execute the program
    {
        let gil = pyo3::Python::acquire_gil();
        let py = gil.python();

        let py_handler = PyHandler::new(&handler);

        if let Err(e) = n3_execute(py, &program, py_handler) {
            e.print_and_set_sys_last_vars(py);
            // manually stop & send the error message
            handler.end_err(e.to_string()).unwrap();
        } else {
            handler.end_ok().unwrap();
        }
    }

    // cleanup
    unsafe { finalize_python() }
}
