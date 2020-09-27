//! Many of this code is from: https://github.com/RustPython/RustPython
//! Different token definitions.
//! Loosely based on token.h from CPython source:
use std::fmt::{self, Write};

/// Python source code can be tokenized in a sequence of these tokens.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Name { name: String },
    UInt { value: u64 },
    Int { value: i64 },
    Float { value: f64 },
    String { value: String },
    Bytes { value: Vec<u8> },
    StartFile,
    Newline,
    Indent,
    Dedent,
    EndOfFile,
    Comma,
    Lpar,
    Rpar,
    Lbrace,
    Rbrace,
    Lbracket,
    Rbracket,
    Colon,
    Equal,
    NodeIdx,
    Dot,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Xor,
    Or,
    BoolYes,
    BoolNo,
    WithDef,
    WithSet,
    LetDef,
    LetBool,
    LetInt,
    LetReal,
    LetDim,
    NodeDef,
    NodeExtern,
    NodeData,
    NodeOptim,
    NodeExec,
    UseDef,
    UseBy,
}

impl fmt::Display for Tok {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Tok::*;
        match self {
            Name { name } => write!(f, "'{}'", name),
            UInt { value } => write!(f, "'{}'", value),
            Int { value } => write!(f, "'{}'", value),
            Float { value } => write!(f, "'{}'", value),
            String { value } => write!(f, "{:?}", value),
            Bytes { value } => {
                write!(f, "b\"")?;
                for i in value {
                    match i {
                        0..=8 => write!(f, "\\x0{}", i)?,
                        9 => f.write_str("\\t")?,
                        10 => f.write_str("\\n")?,
                        11 => write!(f, "\\x0{:x}", i)?,
                        13 => f.write_str("\\r")?,
                        32..=126 => f.write_char(*i as char)?,
                        _ => write!(f, "\\x{:x}", i)?,
                    }
                }
                f.write_str("\"")
            }
            StartFile => f.write_str("StartFile"),
            Newline => f.write_str("Newline"),
            Indent => f.write_str("Indent"),
            Dedent => f.write_str("Dedent"),
            EndOfFile => f.write_str("EOF"),
            Comma => f.write_str("','"),
            Lpar => f.write_str("'('"),
            Rpar => f.write_str("')'"),
            Lbrace => f.write_str("'{'"),
            Rbrace => f.write_str("'}'"),
            Lbracket => f.write_str("'['"),
            Rbracket => f.write_str("']'"),
            Colon => f.write_str("':'"),
            Equal => f.write_str("'='"),
            NodeIdx => f.write_str("'$'"),
            Dot => f.write_str("'.'"),
            Add => f.write_str("'+'"),
            Sub => f.write_str("'-'"),
            Mul => f.write_str("'*'"),
            Div => f.write_str("'/'"),
            Mod => f.write_str("'%'"),
            Pow => f.write_str("'**'"),
            And => f.write_str("'&'"),
            Xor => f.write_str("'^'"),
            Or => f.write_str("'|'"),
            BoolYes => f.write_str("'yes'"),
            BoolNo => f.write_str("'no'"),
            WithDef => f.write_str("'with'"),
            WithSet => f.write_str("'set'"),
            LetDef => f.write_str("'let'"),
            LetBool => f.write_str("'bool'"),
            LetInt => f.write_str("'int'"),
            LetReal => f.write_str("'real'"),
            LetDim => f.write_str("'dim'"),
            NodeDef => f.write_str("'node'"),
            NodeExtern => f.write_str("'extern'"),
            NodeData => f.write_str("'data'"),
            NodeOptim => f.write_str("'optim'"),
            NodeExec => f.write_str("'exec'"),
            UseDef => f.write_str("'use'"),
            UseBy => f.write_str("'by'"),
        }
    }
}
