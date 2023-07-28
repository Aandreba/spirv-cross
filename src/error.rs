use crate::sys;
use std::{error::Error as StdError, fmt::Display};
pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Error {
    pub code: sys::spvc_result,
    pub err_msg: Option<String>,
}

impl Display for sys::spvc_result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            sys::spvc_result::SPVC_SUCCESS => write!(f, "Success"),
            sys::spvc_result::SPVC_ERROR_INVALID_SPIRV => write!(f, "Invalid SPIR-V"),
            sys::spvc_result::SPVC_ERROR_UNSUPPORTED_SPIRV => write!(f, "Unsupported SPIR-V"),
            sys::spvc_result::SPVC_ERROR_OUT_OF_MEMORY => write!(f, "Out of Memory"),
            sys::spvc_result::SPVC_ERROR_INVALID_ARGUMENT => write!(f, "Invalid argument"),
            _ => write!(f, "Unknown"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.err_msg {
            Some(ref msg) => write!(f, "{}: {msg}", self.code),
            None => Display::fmt(&self.code, f),
        }
    }
}

impl From<Error> for sys::spvc_result {
    fn from(value: Error) -> Self {
        return value.code;
    }
}

impl From<sys::spvc_result> for Error {
    fn from(code: sys::spvc_result) -> Self {
        return Self {
            code,
            err_msg: None,
        };
    }
}

impl StdError for Error {
    #[inline]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.code)
    }
}

impl StdError for sys::spvc_result {}
