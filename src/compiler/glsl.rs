use super::{Compiler, GenericCompiler};
use crate::error::Result;
use crate::sys::{self, spvc_compiler_option};
use crate::Context;
use semver::Version;

pub struct GlslCompiler<'a> {
    inner: GenericCompiler<'a>,
}

impl<'a> GlslCompiler<'a> {
    pub fn new(ctx: &'a mut Context, words: &[u32]) -> Result<Self> {
        return Ok(Self {
            inner: GenericCompiler::new(ctx, sys::spvc_backend::SPVC_BACKEND_GLSL, words)?,
        });
    }

    #[inline]
    pub fn into_generic(self) -> GenericCompiler<'a> {
        self.inner
    }

    pub fn semver(self, version: &Version) -> Result<Self> {
        self.version(version.major as u32, version.minor as u32)
    }

    pub fn version(mut self, major: u32, minor: u32) -> Result<Self> {
        let version = (100 * major) + (10 * minor);
        self.inner = self.inner.set_uint(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_VERSION,
            version,
        )?;
        return Ok(self);
    }

    pub fn es(mut self, es_options: impl Into<Option<EsOptions>>) -> Result<Self> {
        let es_options: Option<EsOptions> = es_options.into();
        self.inner = match es_options {
            Some(EsOptions {
                default_float_precision_highp,
                default_int_precision_highp,
            }) => {
                let mut inner = self
                    .inner
                    .set_bool(spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ES, true)?;

                if let Some(default_float_precision_highp) = default_float_precision_highp {
                    inner = inner.set_uint(spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ES_DEFAULT_FLOAT_PRECISION_HIGHP, default_float_precision_highp)?
                }

                if let Some(default_int_precision_highp) = default_int_precision_highp {
                    inner = inner.set_uint(spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ES_DEFAULT_INT_PRECISION_HIGHP, default_int_precision_highp)?
                }

                inner
            }
            None => self
                .inner
                .set_bool(spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ES, false)?,
        };

        return Ok(self);
    }

    pub fn vulkan_semantics(mut self, vulkan_semantics: bool) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_VULKAN_SEMANTICS,
            vulkan_semantics,
        )?;
        return Ok(self);
    }

    pub fn support_nonzero_base_instance(
        mut self,
        support_nonzero_base_instance: bool,
    ) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_SUPPORT_NONZERO_BASE_INSTANCE,
            support_nonzero_base_instance,
        )?;
        return Ok(self);
    }

    pub fn enable_420_pack_extension(self, enable_420_pack_extension: bool) -> Result<Self> {
        return self.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ENABLE_420PACK_EXTENSION,
            enable_420_pack_extension,
        );
    }

    pub fn emit_push_constant_as_uniform_buffer(
        self,
        emit_push_constant_as_uniform_buffer: bool,
    ) -> Result<Self> {
        return self.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_EMIT_PUSH_CONSTANT_AS_UNIFORM_BUFFER,
            emit_push_constant_as_uniform_buffer,
        );
    }

    pub fn emit_uniform_buffer_as_plain_uniforms(
        self,
        emit_uniform_buffer_as_plain_uniforms: bool,
    ) -> Result<Self> {
        return self.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_EMIT_UNIFORM_BUFFER_AS_PLAIN_UNIFORMS,
            emit_uniform_buffer_as_plain_uniforms,
        );
    }

    pub fn force_flattened_io_blocks(self, force_flattened_io_blocks: bool) -> Result<Self> {
        return self.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_FORCE_FLATTENED_IO_BLOCKS,
            force_flattened_io_blocks,
        );
    }

    pub fn ovr_multiview_view_count(self, ovr_multiview_view_count: u32) -> Result<Self> {
        return self.set_uint(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_OVR_MULTIVIEW_VIEW_COUNT,
            ovr_multiview_view_count,
        );
    }

    pub fn enable_row_major_load_workaround(
        self,
        enable_row_major_load_workaround: bool,
    ) -> Result<Self> {
        return self.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ENABLE_ROW_MAJOR_LOAD_WORKAROUND,
            enable_row_major_load_workaround,
        );
    }
}

impl<'a> From<GlslCompiler<'a>> for GenericCompiler<'a> {
    #[inline]
    fn from(value: GlslCompiler<'a>) -> Self {
        value.inner
    }
}

impl<'a> Compiler<'a> for GlslCompiler<'a> {
    fn raw_compile(self) -> Result<&'a std::ffi::CStr> {
        self.inner.raw_compile()
    }

    fn set_uint(
        mut self,
        option: sys::spvc_compiler_option,
        value: std::ffi::c_uint,
    ) -> Result<Self> {
        self.inner = self.inner.set_uint(option, value)?;
        return Ok(self);
    }

    fn set_bool(mut self, option: sys::spvc_compiler_option, value: bool) -> Result<Self> {
        self.inner = self.inner.set_bool(option, value)?;
        return Ok(self);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct EsOptions {
    pub default_float_precision_highp: Option<u32>,
    pub default_int_precision_highp: Option<u32>,
}
