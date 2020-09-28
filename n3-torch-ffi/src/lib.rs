#[macro_use]
extern crate log;

mod data;
mod exec;
pub mod machine;
mod nn;
mod optim;
mod tensor;

macro_rules! add_classes {
    [ $( $ty_nn:path ),*, ] => {
        $(
            pub use $ty_nn;
        )*

        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::wrap_pymodule;
        #[pymodule]
        pub(crate) fn n3(py: Python, m: &PyModule) -> PyResult<()> {
            // make a temporary base module
            #[pymodule]
            fn _base(_py: Python, m: &PyModule) -> PyResult<()> {
                $(
                    m.add_class::<$ty_nn>()?;
                )*
                Ok(())
            }
            let base = wrap_pymodule!(_base)(py);

            let torch = self::machine::Torch(py);
            let module = torch.nn("Module")?;

            $(
                let node_name = &stringify!($ty_nn).split("::").last().unwrap()[1..];
                let node_base = base.getattr(py, node_name)?;

                let node = py.import("builtins")?.get("type")?.call(
                    (node_name, (module, node_base), PyDict::new(py)),
                    None,
                )?;
                m.add(node_name, node)?;
            )*
            Ok(())
        }
    };
}

add_classes![self::nn::Node, self::nn::ExternNode,];
