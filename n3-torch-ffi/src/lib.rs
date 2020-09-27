mod data;
mod exec;
mod machine;
mod nn;
mod optim;

macro_rules! add_classes {
    ( $( $ty:path ),*, ) => {
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

add_classes!(self::nn::Node,);
