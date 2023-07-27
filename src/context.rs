use crate::{sys, Error, Result};
use std::{
    ffi::{c_char, c_void, CStr},
    mem::MaybeUninit,
};

pub struct Context {
    pub(crate) inner: sys::spvc_context,
    error_callback: Option<Box<dyn FnMut()>>,
}

impl Context {
    pub fn new() -> Result<Self> {
        let mut context = MaybeUninit::uninit();
        unsafe {
            return match sys::spvc_context_create(context.as_mut_ptr()) {
                sys::spvc_result::SPVC_SUCCESS => Ok(Self {
                    inner: context.assume_init(),
                    error_callback: None,
                }),
                other => Err(other.into()),
            };
        }
    }

    pub fn set_error_callback<F: FnMut(&CStr)>(&mut self, f: F) {
        unsafe extern "C" fn error_callback_wrapper(user_data: *mut c_void, error: *const c_char) {}
    }

    fn get_error(&self, code: sys::spvc_result) -> Result<()> {
        if code == sys::spvc_result::SPVC_SUCCESS {
            return Ok(());
        }

        unsafe {
            let err_msg = CStr::from_ptr(sys::spvc_context_get_last_error_string(self.inner))
                .to_string_lossy()
                .into_owned();

            return Err(Error {
                code,
                err_msg: Some(err_msg),
            });
        }
    }
}

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        todo!()
    }
}
