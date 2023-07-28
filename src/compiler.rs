use crate::error::Result;
use crate::sys;
use crate::Context;
use crate::ParsedIr;
use std::ffi::c_uint;
use std::ffi::CStr;
use std::mem::MaybeUninit;

pub mod glsl;

pub trait Compiler<'a> {
    fn raw_compile(self) -> Result<&'a CStr>;
}

pub struct GenericCompiler<'a> {
    pub compiler: sys::spvc_compiler,
    pub options: sys::spvc_compiler_options,
    ctx: &'a mut Context,
}

impl<'a> GenericCompiler<'a> {
    pub fn new_parsed<'b>(
        ctx: &'a mut Context,
        backend: sys::spvc_backend,
        parsed_ir: impl Into<ParsedIr<'a, 'b>>,
    ) -> Result<Self>
    where
        'a: 'b,
    {
        let parsed_ir: ParsedIr<'a, 'b> = parsed_ir.into();

        let mut compiler = MaybeUninit::uninit();
        let mut options = MaybeUninit::uninit();

        unsafe {
            ctx.get_error(sys::spvc_context_create_compiler(
                ctx.inner,
                backend,
                parsed_ir.inner,
                parsed_ir.mode(),
                compiler.as_mut_ptr(),
            ))?;

            ctx.get_error(sys::spvc_compiler_create_compiler_options(
                compiler.assume_init_read(),
                options.as_mut_ptr(),
            ))?;

            return Ok(Self {
                compiler: compiler.assume_init(),
                options: options.assume_init(),
                ctx,
            });
        }
    }

    pub fn set_uint(
        &mut self,
        option: sys::spvc_compiler_option,
        value: c_uint,
    ) -> Result<&mut Self> {
        unsafe {
            self.ctx.get_error(sys::spvc_compiler_options_set_uint(
                self.options,
                option,
                value,
            ))?;
        }
        return Ok(self);
    }

    pub fn set_bool(
        &mut self,
        option: sys::spvc_compiler_option,
        value: bool,
    ) -> Result<&mut Self> {
        unsafe {
            self.ctx.get_error(sys::spvc_compiler_options_set_bool(
                self.options,
                option,
                value as sys::spvc_bool,
            ))?;
        }
        return Ok(self);
    }
}

impl<'a> Compiler<'a> for GenericCompiler<'a> {
    fn raw_compile(self) -> Result<&'a CStr> {
        let mut source = MaybeUninit::uninit();
        unsafe {
            self.ctx.get_error(sys::spvc_compiler_compile(
                self.compiler,
                source.as_mut_ptr(),
            ))?;

            return Ok(CStr::from_ptr(source.assume_init()));
        }
    }
}

impl sys::spvc_compiler_option {
    pub fn is_common(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_COMMON_BIT == 1;
    }

    pub fn is_glsl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_GLSL_BIT == 1;
    }

    pub fn is_hlsl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_HLSL_BIT == 1;
    }

    pub fn is_msl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_MSL_BIT == 1;
    }
}

unsafe impl<'a> Send for GenericCompiler<'a> {}
unsafe impl<'a> Sync for GenericCompiler<'a> {}
