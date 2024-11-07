use nom::{error::Error, Err};

#[derive(Debug)]
pub enum MyErr {
    FileError(std::io::Error),
    ParseError(Err<Error<String>>),
}

impl std::fmt::Display for MyErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyErr::FileError(e) => write!(f, "File IO error: {e:?}"),
            MyErr::ParseError(e) => write!(f, "Parser error: {e:?}"),
        }
    }
}

impl From<Err<Error<&str>>> for MyErr {
    fn from(e: Err<Error<&str>>) -> MyErr {
        let inner_err = match e {
            Err::Incomplete(n) => Err::Incomplete(n),
            Err::Error(e) => Err::Error(conv_error(&e)),
            Err::Failure(e) => Err::Failure(conv_error(&e)),
        };
        MyErr::ParseError(inner_err)
    }
}

impl From<std::io::Error> for MyErr {
    fn from(e: std::io::Error) -> MyErr {
        MyErr::FileError(e)
    }
}

fn conv_error(e: &Error<&str>) -> Error<String> {
    Error {
        input: e.input.to_owned(),
        code: e.code,
    }
}
