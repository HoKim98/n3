#![deny(clippy::all)]

pub extern crate n3_parser_ast as ast;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(#[allow(clippy::all)] pub grammar); // synthesized by LALRPOP

pub mod error;

mod lexer;
mod location;
mod parser;
mod token;

pub use self::parser::Parser;
