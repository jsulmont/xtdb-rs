use crate::xtql::parse::Rule;
use std::fmt::{self, Debug};

#[derive(Debug)]
pub enum Error {
    PestParse(Box<pest::error::Error<Rule>>),
    IO(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            PestParse(err) => match err.line_col {
                pest::error::LineColLocation::Pos(pos) => {
                    write!(f, "Parse Error: at {}:{}", pos.0, pos.1)
                }
                pest::error::LineColLocation::Span(start, end) => {
                    write!(
                        f,
                        "Parse Error: at {}:{} to {}:{}",
                        start.0, start.1, end.0, end.1
                    )
                }
            },

            IO(err) => write!(f, "IO Error: {}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match *self {
            PestParse(ref err) => Some(err),
            IO(ref err) => Some(err),
        }
    }
}

impl From<pest::error::Error<Rule>> for Error {
    fn from(err: pest::error::Error<Rule>) -> Self {
        Error::PestParse(Box::new(err))
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}
