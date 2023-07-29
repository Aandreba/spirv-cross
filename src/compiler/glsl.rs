use super::{Compiler, GenericCompiler};
use crate::error::Result;
use crate::sys::{self, spvc_compiler_option};
use crate::Context;
use docfg::docfg;
use semver::Version;

#[docfg(feature = "glsl")]
pub struct GlslCompiler<'a> {
    inner: GenericCompiler<'a>,
}

#[docfg(feature = "glsl")]
impl<'a> GlslCompiler<'a> {
    pub fn new<'b>(ctx: &'a mut Context, words: &[u32]) -> Result<Self>
    where
        'a: 'b,
    {
        return Ok(Self {
            inner: GenericCompiler::new(ctx, sys::spvc_backend::SPVC_BACKEND_GLSL, words)?,
        });
    }

    #[inline]
    pub fn into_generic(self) -> GenericCompiler<'a> {
        self.inner
    }

    pub fn semver(self, version: Version) -> Result<Self> {
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

    pub fn enable_420_pack_extension(mut self, enable_420_pack_extension: bool) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ENABLE_420PACK_EXTENSION,
            enable_420_pack_extension,
        )?;
        return Ok(self);
    }

    pub fn emit_push_constant_as_uniform_buffer(
        mut self,
        emit_push_constant_as_uniform_buffer: bool,
    ) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_EMIT_PUSH_CONSTANT_AS_UNIFORM_BUFFER,
            emit_push_constant_as_uniform_buffer,
        )?;
        return Ok(self);
    }

    pub fn emit_uniform_buffer_as_plain_uniforms(
        mut self,
        emit_uniform_buffer_as_plain_uniforms: bool,
    ) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_EMIT_UNIFORM_BUFFER_AS_PLAIN_UNIFORMS,
            emit_uniform_buffer_as_plain_uniforms,
        )?;
        return Ok(self);
    }

    pub fn force_flattened_io_blocks(mut self, force_flattened_io_blocks: bool) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_FORCE_FLATTENED_IO_BLOCKS,
            force_flattened_io_blocks,
        )?;
        return Ok(self);
    }

    pub fn ovr_multiview_view_count(
        mut self,
        ovr_multiview_view_count: u32,
    ) -> Result<Self> {
        self.inner = self.inner.set_uint(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_OVR_MULTIVIEW_VIEW_COUNT,
            ovr_multiview_view_count,
        )?;
        return Ok(self);
    }

    pub fn enable_row_major_load_workaround(
        mut self,
        enable_row_major_load_workaround: bool,
    ) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ENABLE_ROW_MAJOR_LOAD_WORKAROUND,
            enable_row_major_load_workaround,
        )?;
        return Ok(self);
    }
}

#[docfg(feature = "glsl")]
impl<'a> From<GlslCompiler<'a>> for GenericCompiler<'a> {
    #[inline]
    fn from(value: GlslCompiler<'a>) -> Self {
        value.inner
    }
}

#[docfg(feature = "glsl")]
impl<'a> Compiler<'a> for GlslCompiler<'a> {
    fn raw_compile(self) -> Result<&'a std::ffi::CStr> {
        self.inner.raw_compile()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct EsOptions {
    pub default_float_precision_highp: Option<u32>,
    pub default_int_precision_highp: Option<u32>,
}
