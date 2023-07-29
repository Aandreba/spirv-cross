use super::{Compiler, GenericCompiler};
use crate::{error::Result, sys, Context};
use docfg::docfg;
use semver::Version;

/// High Level Language compiler.
pub struct HlslCompiler<'a> {
    inner: GenericCompiler<'a>,
}

impl<'a> HlslCompiler<'a> {
    pub fn new(ctx: &'a mut Context, words: &[u32]) -> Result<Self> {
        return Ok(Self {
            inner: GenericCompiler::new(ctx, sys::spvc_backend::SPVC_BACKEND_HLSL, words)?,
        });
    }

    #[inline]
    pub fn into_generic(self) -> GenericCompiler<'a> {
        self.inner
    }

    pub fn shader_model_semver(self, version: &Version) -> Result<Self> {
        self.shader_model(version.major as u32, version.minor as u32)
    }

    pub fn shader_model(self, major: u32, minor: u32) -> Result<Self> {
        let version = (10 * major) + minor;
        self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_SHADER_MODEL,
            version,
        )
    }

    pub fn point_size_compat(self, point_size_compat: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_POINT_SIZE_COMPAT,
            point_size_compat,
        )
    }

    pub fn point_coord_compat(self, point_coord_compat: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_POINT_COORD_COMPAT,
            point_coord_compat,
        )
    }

    pub fn support_nonzero_base_vertex_base_instance(
        self,
        support_nonzero_base_vertex_base_instance: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_SUPPORT_NONZERO_BASE_VERTEX_BASE_INSTANCE,
            support_nonzero_base_vertex_base_instance,
        )
    }

    pub fn force_storage_buffer_as_uav(self, force_storage_buffer_as_uav: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_FORCE_STORAGE_BUFFER_AS_UAV,
            force_storage_buffer_as_uav,
        )
    }

    pub fn nonwritable_uav_texture_as_srv(
        self,
        nonwritable_uav_texture_as_srv: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_NONWRITABLE_UAV_TEXTURE_AS_SRV,
            nonwritable_uav_texture_as_srv,
        )
    }

    pub fn enable_16bit_types(self, enable_16bit_types: bool) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_ENABLE_16BIT_TYPES,
            enable_16bit_types,
        )
    }

    pub fn flatten_matrix_vertex_input_semantics(
        self,
        flatten_matrix_vertex_input_semantics: bool,
    ) -> Result<Self> {
        self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_HLSL_FLATTEN_MATRIX_VERTEX_INPUT_SEMANTICS,
            flatten_matrix_vertex_input_semantics,
        )
    }
}

impl<'a> From<HlslCompiler<'a>> for GenericCompiler<'a> {
    #[inline]
    fn from(value: HlslCompiler<'a>) -> Self {
        value.inner
    }
}

impl<'a> Compiler<'a> for HlslCompiler<'a> {
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
