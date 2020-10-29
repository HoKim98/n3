#![deny(clippy::all)]

pub extern crate pyo3;
pub extern crate pyo3_mp;

mod data;
mod error;
mod exec;
mod nn;
mod optim;
mod tensor;
mod torch;

pub use self::torch::Torch;

#[cfg(not(test))]
macro_rules! add_classes_dyn {
    (
        $( Some($base:path) => [$( $ty_child:path ),*,], ),*
           None             => [$( $ty      :path ),*,],
    ) => {
        $(
            $(
                pub use $ty_child;
            )*
        )*
        $(
            pub use $ty;
        )*

        use pyo3::prelude::*;
        use pyo3::types::PyDict;
        use pyo3::wrap_pymodule;

        #[pymodule]
        pub(crate) fn n3_torch_ffi(py: Python, m: &PyModule) -> PyResult<()> {
            let builtins = py.import("builtins")?;
            let ty = builtins.get("type")?;
            $(
                {
                    // make a temporary base module
                    #[pymodule]
                    fn _base(_py: Python, m: &PyModule) -> PyResult<()> {
                        $(
                            m.add_class::<$ty_child>()?;
                        )*
                        Ok(())
                    }

                    let base = wrap_pymodule!(_base)(py);
                    let mut base_path: Vec<_> = stringify!($base).split("::").collect();
                    let base_name = base_path.pop().unwrap();

                    // get the base module & type
                    let base_module = py.import(&base_path.join("."))?;
                    let base_ty = base_module.get(base_name)?.into_py(py);

                    $(
                        let node_name = stringify!($ty_child).split("::").last().unwrap();
                        let node_base = base.getattr(py, &node_name)?;
                        let node = ty.call(
                            (node_name, (&base_ty, node_base), PyDict::new(py)),
                            None,
                        )?;
                        m.add(node_name, node)?;
                    )*
                }
            )*
            $(
                m.add_class::<$ty>()?;
            )*
            Ok(())
        }
    };
}

#[cfg(not(test))]
add_classes_dyn!(
    Some(torch::nn::Module) => [
        self::nn::Node,
        self::nn::ExternNode,
    ],
    None => [
        self::exec::Trainer,
    ],
);
