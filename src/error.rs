use peg;
use std::convert::From;
use std::fmt::Debug;
use std::{fmt, io};

#[derive(Debug)]
pub enum CompileError {
    ParseError(String),
    SyntaxError(String),
    TypeError(String),
    IoError(io::Error),
    FmtError(fmt::Error),
}

impl From<io::Error> for CompileError {
    fn from(e: io::Error) -> Self {
        CompileError::IoError(e)
    }
}

impl From<fmt::Error> for CompileError {
    fn from(e: fmt::Error) -> Self {
        CompileError::FmtError(e)
    }
}

impl<L: Debug> From<peg::error::ParseError<L>> for CompileError {
    fn from(e: peg::error::ParseError<L>) -> Self {
        CompileError::SyntaxError(format!("{:?}", e))
    }
}
