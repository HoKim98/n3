#![cfg_attr(feature = "test-nightly", feature(is_sorted))]
#![deny(clippy::all)]

mod n3_std;

mod cache;
mod code;
mod context;
mod error;
mod execs;
mod externs;
mod graph;
mod nodes;
mod seed;
mod tensor;
mod variable;

pub use n3_parser::ast;

pub use self::code::{Code, CodeData, CodeType};
pub use self::error::{Error, Result};
pub use self::execs::{dirs, ExecRoot, GlobalVars, Program};
pub use self::externs::{ExternCode, PythonScripts};
pub use self::graph::ToValues;
pub use self::nodes::NodeCode;

use n3_parser::Parser;

#[cfg(test)]
mod tests_recon {
    use std::fs;

    fn recon(source: &str) {
        let parser = super::Parser::default();

        let source_recon1 = format!("{:?}", parser.parse_file(source).unwrap());
        println!("{}", &source_recon1);
        let source_recon2 = format!("{:?}", parser.parse_file(&source_recon1).unwrap());

        assert_eq!(source_recon1, source_recon2);
    }

    #[test]
    fn test_dummy() {
        let source = fs::read_to_string("tests/data/nodes/__user__/sample/dummy.n3").unwrap();

        recon(&source);
    }

    #[test]
    fn test_all_externs() {
        let path = std::path::PathBuf::from("../n3-torch-ffi-python/n3");
        for source in super::n3_std::get_sources(&path).values() {
            recon(&source);
        }
    }
}
