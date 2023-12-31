use crate::sys;
use crate::Context;
use crate::Result;
use std::ffi::c_uint;
use std::ffi::CStr;
use std::mem::MaybeUninit;

/// OpenGL Shading Language compiler & options
#[cfg(feature = "glsl")]
#[cfg_attr(docsrs, doc(cfg(feature = "glsl")))]
pub mod glsl;

/// High Level Shading Language compiler & options
#[cfg(feature = "hlsl")]
#[cfg_attr(docsrs, doc(cfg(feature = "hlsl")))]
pub mod hlsl;

/// Metal Shading Language compiler & options
#[cfg(feature = "msl")]
#[cfg_attr(docsrs, doc(cfg(feature = "msl")))]
pub mod msl;

#[cfg(feature = "glsl")]
#[doc(inline)]
pub use glsl::GlslCompiler;
#[cfg(feature = "hlsl")]
#[doc(inline)]
pub use hlsl::HlslCompiler;
#[cfg(feature = "msl")]
#[doc(inline)]
pub use msl::MslCompiler;

/// A SPIRV-Cross compiler to an unkown target.
pub trait Compiler<'a>: Sized {
    /// Sets the compiler's `option` property to the specified integer value.
    fn set_uint(self, option: sys::spvc_compiler_option, value: c_uint) -> Result<Self>;

    /// Sets the compiler's `option` property to the specified boolean value.
    fn set_bool(self, option: sys::spvc_compiler_option, value: bool) -> Result<Self>;

    /// Compiles the program with the specified options, returning a reference to the context's
    /// resulting C string.
    ///
    /// This method is usefull if you want to avoid extra allocations,
    /// or intend to pass the result to another C function.
    fn raw_compile(self) -> Result<&'a CStr>;

    fn force_temporary(self, force_temporary: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_FORCE_TEMPORARY,
            force_temporary,
        )
    }

    fn flatten_multidimensional_arrays(
        self,
        flatten_multidimensional_arrays: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_FLATTEN_MULTIDIMENSIONAL_ARRAYS,
            flatten_multidimensional_arrays,
        )
    }

    fn fixup_depth_convention(self, fixup_depth_convention: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_FIXUP_DEPTH_CONVENTION,
            fixup_depth_convention,
        )
    }

    fn flip_vertex_y(self, flip_vertex_y: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_FLIP_VERTEX_Y,
            flip_vertex_y,
        )
    }

    fn emit_line_directives(self, emit_line_directives: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_EMIT_LINE_DIRECTIVES,
            emit_line_directives,
        )
    }

    fn enable_storage_image_qualifier_deduction(
        self,
        enable_storage_image_qualifier_deduction: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_ENABLE_STORAGE_IMAGE_QUALIFIER_DEDUCTION,
            enable_storage_image_qualifier_deduction,
        )
    }

    fn force_zero_initialized_variables(
        self,
        force_zero_initialized_variables: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_FORCE_ZERO_INITIALIZED_VARIABLES,
            force_zero_initialized_variables,
        )
    }

    /// Compiles the program with the specified options, returning a UTF-8 encoded copy of the result.
    ///
    /// This method makes use of `CStr::to_string_lossy`, so if the resulting source code returns invalid UTF-8, it will be replaced with the [`REPLACEMENT_CHARACTER`](std::char::REPLACEMENT_CHARACTER) (�).
    fn compile(self) -> Result<String> {
        let src = self.raw_compile()?;
        return Ok(src.to_string_lossy().into_owned());
    }
}

/// A SPIRV-Cross compiler to a manually specified target.
pub struct GenericCompiler<'a> {
    pub compiler: sys::spvc_compiler,
    pub options: sys::spvc_compiler_options,
    ctx: &'a mut Context,
}

impl<'a> GenericCompiler<'a> {
    /// Creates a new [`GenericCompiler`]
    pub fn new(ctx: &'a mut Context, backend: sys::spvc_backend, words: &[u32]) -> Result<Self> {
        let mut parsed_ir = MaybeUninit::uninit();
        let mut compiler = MaybeUninit::uninit();
        let mut options = MaybeUninit::uninit();

        unsafe {
            ctx.get_error(sys::spvc_context_parse_spirv(
                ctx.inner,
                words.as_ptr(),
                words.len(),
                parsed_ir.as_mut_ptr(),
            ))?;

            ctx.get_error(sys::spvc_context_create_compiler(
                ctx.inner,
                backend,
                parsed_ir.assume_init(),
                sys::spvc_capture_mode::SPVC_CAPTURE_MODE_TAKE_OWNERSHIP,
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
}

impl<'a> Compiler<'a> for GenericCompiler<'a> {
    fn set_uint(self, option: sys::spvc_compiler_option, value: c_uint) -> Result<Self> {
        unsafe {
            self.ctx.get_error(sys::spvc_compiler_options_set_uint(
                self.options,
                option,
                value,
            ))?;
        }
        return Ok(self);
    }

    fn set_bool(self, option: sys::spvc_compiler_option, value: bool) -> Result<Self> {
        unsafe {
            self.ctx.get_error(sys::spvc_compiler_options_set_bool(
                self.options,
                option,
                value as sys::spvc_bool,
            ))?;
        }
        return Ok(self);
    }

    fn raw_compile(self) -> Result<&'a CStr> {
        let mut source = MaybeUninit::uninit();
        unsafe {
            self.ctx
                .get_error(sys::spvc_compiler_install_compiler_options(
                    self.compiler,
                    self.options,
                ))?;

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
        return (self as u32) & sys::SPVC_COMPILER_OPTION_COMMON_BIT != 0;
    }

    pub fn is_glsl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_GLSL_BIT != 0;
    }

    pub fn is_hlsl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_HLSL_BIT != 0;
    }

    pub fn is_msl(self) -> bool {
        return (self as u32) & sys::SPVC_COMPILER_OPTION_MSL_BIT != 0;
    }
}

unsafe impl<'a> Send for GenericCompiler<'a> {}
unsafe impl<'a> Sync for GenericCompiler<'a> {}
