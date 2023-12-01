use crate::{
    error::{Error, Result},
    sys,
};
use std::{ffi::CStr, mem::MaybeUninit, ops::Deref};

/// Manager of SPIRV-Cross resources
pub struct Context {
    pub inner: sys::spvc_context,
    error_callback: Option<Box<dyn FnMut(&CStr)>>,
}

impl Context {
    /// Creates a new empty context.
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

    /// Sets a new callback function to be called whenever a context returns an error.
    pub fn set_error_callback<F: 'static + FnMut(&CStr)>(&mut self, f: F) {
        unsafe extern "C" fn error_callback_wrapper<F: 'static + FnMut(&CStr)>(
            user_data: *mut std::ffi::c_void,
            error: *const std::ffi::c_char,
        ) {
            let f = &mut *user_data.cast::<F>();
            (f)(CStr::from_ptr(error));
        }

        let f = Box::<F>::new(f);
        unsafe {
            sys::spvc_context_set_error_callback(
                self.inner,
                Some(error_callback_wrapper::<F>),
                f.deref() as *const F as *mut std::ffi::c_void,
            );
            self.error_callback = Some(f as Box<dyn FnMut(&CStr)>);
        }
    }

    /// Release all the resources associated to this context.
    ///
    /// This method is safe because the shared reference ensures no other part of the code
    /// has access to the context.
    #[inline]
    pub fn release_allocations(&mut self) {
        unsafe { sys::spvc_context_release_allocations(self.inner) }
    }

    pub(crate) fn get_error(&self, code: sys::spvc_result) -> Result<()> {
        if code == sys::spvc_result::SPVC_SUCCESS {
            return Ok(());
        }

        #[cfg(debug_assertions)]
        println!("{}", std::backtrace::Backtrace::capture());

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
        unsafe { sys::spvc_context_destroy(self.inner) }
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}
