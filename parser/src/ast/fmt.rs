use std::ops::Deref;

const INDENT: &'static str = "    ";

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

    pub fn sibling<C>(&self, child: &'a C) -> FmtGuard<'a, C> {
        FmtGuard {
            inner: child,
            depth: self.depth,
        }
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
