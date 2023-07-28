use super::{Compiler, GenericCompiler};
use crate::error::Result;
use crate::sys::{self, spvc_compiler_option};
use crate::{Context, ParsedIr};
use semver::Version;

pub struct GlslCompiler<'a> {
    inner: GenericCompiler<'a>,
}

impl<'a> GlslCompiler<'a> {
    pub fn new<'b>(ctx: &'a mut Context, parsed_ir: impl Into<ParsedIr<'a, 'b>>) -> Result<Self>
    where
        'a: 'b,
    {
        return Ok(Self {
            inner: GenericCompiler::new(ctx, sys::spvc_backend::SPVC_BACKEND_GLSL, parsed_ir)?,
        });
    }

    #[inline]
    pub fn into_generic(self) -> GenericCompiler<'a> {
        self.inner
    }

    pub fn set_semver(self, version: Version) -> Result<Self> {
        self.set_version(version.major as u32, version.minor as u32)
    }

    pub fn set_version(mut self, major: u32, minor: u32) -> Result<Self> {
        let version = (100 * major) + (10 * minor);
        self.inner = self.inner.set_uint(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_VERSION,
            version,
        )?;
        return Ok(self);
    }

    pub fn set_es(mut self, is_es: bool) -> Result<Self> {
        self.inner = self
            .inner
            .set_bool(spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_ES, is_es)?;
        return Ok(self);
    }

    pub fn set_vulkan_semantics(mut self, vulkan_semantics: bool) -> Result<Self> {
        self.inner = self.inner.set_bool(
            spvc_compiler_option::SPVC_COMPILER_OPTION_GLSL_VULKAN_SEMANTICS,
            vulkan_semantics,
        )?;
        return Ok(self);
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
}
