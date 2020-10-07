use std::iter;

use crate::ast;
use crate::error::ParseError;
use crate::grammar;
use crate::lexer;
use crate::token;

pub struct Parser {
    inner: grammar::FileInputParser,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            inner: grammar::FileInputParser::new(),
        }
    }
}

impl Parser {
    pub fn parse_file(&self, source: &str) -> Result<ast::File, ParseError> {
        let lxr = lexer::make_tokenizer(source);
        let marker_token = (
            Default::default(),
            token::Tok::StartFile,
            Default::default(),
        );
        let tokenizer = iter::once(Ok(marker_token)).chain(lxr);

        self.inner.parse(tokenizer).map_err(ParseError::from)
    }
}
