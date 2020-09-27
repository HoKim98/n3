use std::marker::PhantomData;

use pyo3::prelude::*;
use pyo3::type_object::PyBorrowFlagLayout;
use pyo3::PyClass;

pub struct PyBox<T>
where
    T: PyClass,
{
    inner: PyObject,
    _phantom: PhantomData<T>,
}

impl<T> PyBox<T>
where
    T: PyClass,
    T::BaseLayout: PyBorrowFlagLayout<T::BaseType>,
{
    pub fn new(py: Python, cls: impl Into<PyClassInitializer<T>>) -> PyResult<Self> {
        Ok(Self {
            inner: PyCell::new(py, cls)?.to_object(py).into_py(py),
            _phantom: Default::default(),
        })
    }
}

impl<'a, T> PyBox<T>
where
    T: PyClass + 'static,
{
    pub fn as_ref(&'a self, py: Python<'a>) -> PyRef<'a, T> {
        self.inner
            .as_ref(py)
            .downcast::<PyCell<T>>()
            .unwrap()
            .borrow()
    }

    pub fn as_mut(&'a self, py: Python<'a>) -> PyRefMut<'a, T> {
        self.inner
            .as_ref(py)
            .downcast::<PyCell<T>>()
            .unwrap()
            .borrow_mut()
    }
}
