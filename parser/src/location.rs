//! Many of this code is from: https://github.com/RustPython/RustPython
//! Datatypes to support source location information.

use std::fmt;

/// A location somewhere in the sourcecode.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Location {
    row: usize,
    column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {} column {}", self.row, self.column)
    }
}

impl Location {
    pub fn visualize(&self, desc: &str) -> String {
        format!(
            "{}↑\n{}{}",
            " ".repeat(self.column - 1),
            " ".repeat(self.column - 1),
            desc
        )
    }
}

impl Location {
    pub fn empty() -> Self {
        Location::new(0, 0)
    }

    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn reset(&mut self) {
        self.row = 1;
        self.column = 1;
    }

    pub fn go_right(&mut self) {
        self.column += 1;
    }

    pub fn newline(&mut self) {
        self.row += 1;
        self.column = 1;
    }
}
