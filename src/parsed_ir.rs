use crate::{error::Result, sys, Context};
use std::{marker::PhantomData, mem::MaybeUninit, ops::Deref};

pub enum ParsedIr<'a, 'b> {
    Owned(OwnedParsedIr<'a>),
    Borrowed(&'b OwnedParsedIr<'a>),
}

impl<'a, 'b> ParsedIr<'a, 'b> {
    pub fn new(ctx: &'a mut Context, words: &[u32]) -> Result<Self> {
        OwnedParsedIr::new(ctx, words).map(Self::Owned)
    }

    pub fn mode(&self) -> sys::spvc_capture_mode {
        match self {
            ParsedIr::Owned(_) => sys::spvc_capture_mode::SPVC_CAPTURE_MODE_COPY,
            ParsedIr::Borrowed(_) => sys::spvc_capture_mode::SPVC_CAPTURE_MODE_TAKE_OWNERSHIP,
        }
    }
}

impl<'a, 'b> Deref for ParsedIr<'a, 'b> {
    type Target = OwnedParsedIr<'a>;

    fn deref(&self) -> &Self::Target {
        match self {
            ParsedIr::Owned(x) => x,
            ParsedIr::Borrowed(x) => x,
        }
    }
}

impl<'a, 'b> From<OwnedParsedIr<'a>> for ParsedIr<'a, 'b> {
    fn from(value: OwnedParsedIr<'a>) -> Self {
        Self::Owned(value)
    }
}

impl<'a, 'b> From<&'b OwnedParsedIr<'a>> for ParsedIr<'a, 'b> {
    fn from(value: &'b OwnedParsedIr<'a>) -> Self {
        Self::Borrowed(value)
    }
}

#[repr(transparent)]
pub struct OwnedParsedIr<'a> {
    pub inner: sys::spvc_parsed_ir,
    ctx: PhantomData<&'a Context>,
}

impl<'a> OwnedParsedIr<'a> {
    pub fn new(ctx: &'a mut Context, words: &[u32]) -> Result<Self> {
        let mut parsed_ir = MaybeUninit::uninit();
        unsafe {
            ctx.get_error(sys::spvc_context_parse_spirv(
                ctx.inner,
                words.as_ptr(),
                words.len(),
                parsed_ir.as_mut_ptr(),
            ))?;

            return Ok(Self {
                inner: parsed_ir.assume_init(),
                ctx: PhantomData,
            });
        }
    }
}

unsafe impl<'a> Send for OwnedParsedIr<'a> {}
unsafe impl<'a> Sync for OwnedParsedIr<'a> {}
