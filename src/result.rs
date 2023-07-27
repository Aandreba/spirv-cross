use crate::sys;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug, Clone)]
pub struct Error {
    pub code: sys::spvc_result,
    pub err_msg: Option<String>,
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
