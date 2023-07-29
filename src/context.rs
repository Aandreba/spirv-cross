use crate::{
    error::{Error, Result},
    sys,
};
use std::{backtrace::Backtrace, ffi::CStr, mem::MaybeUninit};

/// Manager of SPIRV-Cross resources
pub struct Context {
    pub inner: sys::spvc_context,
    #[cfg(not(feature = "nightly"))]
    error_callback: Option<Box<Box<dyn FnMut(&CStr)>>>,
    #[cfg(feature = "nightly")]
    error_callback: Option<std::boxed::ThinBox<dyn FnMut(&CStr)>>,
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
    ///
    /// If `nightly` is enabled, this function is optimized to a single memory allocation.
    /// Otherwise, two memory allocations are used to store the callback.
    pub fn set_error_callback<F: 'static + FnMut(&CStr)>(&mut self, f: F) {
        unsafe extern "C" fn error_callback_wrapper(
            user_data: *mut std::ffi::c_void,
            error: *const std::ffi::c_char,
        ) {
            cfg_if::cfg_if! {
            if #[cfg(feature = "nightly")] {
                let mut f = core::mem::ManuallyDrop::new(core::mem::transmute::<
                    _,
                    ThinBox<dyn FnMut(&CStr)>,
                    >(user_data));
                } else {
                    let f = &mut *(user_data as *mut Box<dyn FnMut(&CStr)>);
                }
            }

            (f)(CStr::from_ptr(error));
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "nightly")] {
                use std::boxed::ThinBox;
                let f = ThinBox::<dyn FnMut(&CStr)>::new_unsize(f);
                let user_data = unsafe { core::mem::transmute_copy(&f) };
            } else {
                let mut f = Box::new(Box::new(f) as Box<dyn FnMut(&CStr)>);
                let user_data = (&mut f as &mut Box<dyn FnMut(&CStr)>) as *mut Box<dyn FnMut(&CStr)> as *mut std::ffi::c_void;
            }
        }

        unsafe {
            sys::spvc_context_set_error_callback(
                self.inner,
                Some(error_callback_wrapper),
                user_data,
            );
            self.error_callback = Some(f);
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
        println!("{}", Backtrace::capture());

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
