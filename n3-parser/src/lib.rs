#![deny(clippy::all)]

#[macro_use]
extern crate lalrpop_util;
#[macro_use]
extern crate log;

lalrpop_mod!(#[allow(clippy::all)] pub grammar); // synthesized by LALRPOP

pub mod ast;
pub mod error;
mod lexer;
mod location;
mod parser;
mod token;

pub use self::parser::Parser;
