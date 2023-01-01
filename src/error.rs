use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorType {
    WrongBufferSize(usize, usize),
    String(String),
    RustNulError(std::ffi::NulError)
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::WrongBufferSize(expected, actual) => {
                write!(f, "buffer was wrong size- expected size of {} but received {}", expected, actual)
            },
            ErrorType::String(str) => write!(f, "{}", str),
            ErrorType::RustNulError(..) => write!(f, "{}", self.source().unwrap()) 
        }
    }
}

impl Error for ErrorType {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            ErrorType::WrongBufferSize(..) => None,
            ErrorType::String(..) => None,
            ErrorType::RustNulError(ref e) => Some(e)
        }
    }
}

macro_rules! impl_from_for_errortype {
    ($t:ty, $variant:ident) => {
        impl From<$t> for ErrorType {
            fn from(err: $t) -> ErrorType {
                ErrorType::$variant(err)
            }
        }
    };
}

impl_from_for_errortype!(String, String);
impl_from_for_errortype!(std::ffi::NulError, RustNulError);