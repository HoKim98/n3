#[macro_use]
extern crate log;

mod data;
mod exec;
pub mod machine;
mod nn;
mod optim;
mod tensor;

macro_rules! add_classes {
    [ $( $ty:path ),*, ] => {
        $(
            pub use $ty;
        )*

        use pyo3::prelude::*;

        #[pymodule]
        pub(crate) fn n3(_py: Python, m: &PyModule) -> PyResult<()> {
            $(
                m.add_class::<$ty>()?;
            )*
            Ok(())
        }
    };
}

add_classes![self::nn::Node, self::nn::ExternNode,];
