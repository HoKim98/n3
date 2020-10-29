use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyCFunction};
use pyo3::{wrap_pyfunction, wrap_pymodule};

use n3_machine::Program;
pub use n3_torch_ffi::PyInit_n3_torch_ffi;
use n3_torch_ffi::{pyo3, Torch};

pub fn n3_execute_wrapper(py: Python) -> PyResult<&PyCFunction> {
    wrap_pyfunction!(n3_execute)(py)
}

#[pyfunction]
fn n3_execute(py: Python, id: usize, machine: &str, program: &Program) -> PyResult<()> {
    let program = n3_builder::Program::load(program).unwrap();
    dbg!(id, machine);

    let builtins = py.import("builtins")?.into_py(py);
    let torch = Torch(py);

    let n3 = wrap_pymodule!(n3_torch_ffi)(py);
    let nn = torch.this("nn")?.into_py(py);
    let zeros = torch.this("zeros")?.into_py(py);

    py.run(
        "print('hello world')",
        //         r#"
        // class MyExternNode(n3_torch_ffi.ExternNode):
        //     def __init__(self):
        //         super().__init__()
        //         self.inner1 = nn.Linear(32, 64)
        //         self.inner2 = nn.Linear(64, 10, bias=False)

        //     def forward(self, x):
        //         x = self.inner1(x)
        //         x = self.inner2(x)
        //         return x

        // node = MyExternNode()
        // node.init_node({}, {})

        // assert len(list(node.parameters())) == 3

        // x = zeros(3, 32)
        // y = node(x)
        // assert y.shape == (3, 10)

        // "#,
        Some(
            [
                ("__builtins__", builtins),
                ("n3_torch_ffi", n3),
                ("nn", nn),
                ("zeros", zeros),
            ]
            .into_py_dict(py),
        ),
        None,
    )?;

    todo!();
}
