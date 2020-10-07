use std::ops::Deref;

const INDENT: &str = "    ";

pub struct FmtGuard<'a, T> {
    inner: &'a T,
    depth: usize,
}

impl<'a, T> FmtGuard<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner, depth: 0 }
    }

    pub fn indent(&self) -> String {
        INDENT.repeat(self.depth)
    }

    pub fn child<C>(&self, child: &'a C) -> FmtGuard<'a, C> {
        FmtGuard {
            inner: child,
            depth: self.depth + 1,
        }
    }
}

impl<'a, T> Deref for FmtGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

#[macro_export]
macro_rules! impl_debug_no_guard(
    ($t:ty) => {
        impl std::fmt::Debug for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                FmtGuard::new(self).fmt(f)
            }
        }
    }
);
