use crate::{
    error::{Error, Result},
    sys,
};
use docfg::docfg;
use std::{ffi::CStr, mem::MaybeUninit};

pub struct Context {
    pub inner: sys::spvc_context,
    #[cfg(feature = "nightly")]
    error_callback: Option<ThinBox<dyn FnMut(&CStr)>>,
}

impl Context {
    pub fn new() -> Result<Self> {
        let mut context = MaybeUninit::uninit();
        unsafe {
            return match sys::spvc_context_create(context.as_mut_ptr()) {
                sys::spvc_result::SPVC_SUCCESS => Ok(Self {
                    inner: context.assume_init(),
                    #[cfg(feature = "nightly")]
                    error_callback: None,
                }),
                other => Err(other.into()),
            };
        }
    }

    #[docfg(feature = "nightly")]
    pub fn set_error_callback<F: FnMut(&CStr)>(&mut self, f: F) {
        use std::boxed::ThinBox;

        unsafe extern "C" fn error_callback_wrapper(
            user_data: *mut std::ffi::c_void,
            error: *const std::ffi::c_char,
        ) {
            let mut f =
                ManuallyDrop::new(core::mem::transmute::<_, ThinBox<dyn FnMut(&CStr)>>(src));
            (f)(CStr::from_ptr(error));
        }

        let f = ThinBox::<dyn FnMut(&CStr)>::new_unsize(f);
        unsafe {
            sys::spvc_context_set_error_callback(
                self.inner,
                Some(error_callback_wrapper),
                core::mem::transmute_copy(&f),
            );
            self.error_callback = Some(f);
        }
    }

    #[inline]
    pub unsafe fn release_allocations(&mut self) {
        unsafe { sys::spvc_context_release_allocations(self.inner) }
    }

    pub(crate) fn get_error(&self, code: sys::spvc_result) -> Result<()> {
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
        unsafe { sys::spvc_context_destroy(self.inner) }
    }
}

unsafe impl Send for Context {}
unsafe impl Sync for Context {}
