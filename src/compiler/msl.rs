use super::{Compiler, GenericCompiler};
use crate::{error::Result, sys, Context};
use docfg::docfg;
use semver::Version;

pub struct MslCompiler<'a> {
    inner: GenericCompiler<'a>,
}

impl<'a> MslCompiler<'a> {
    pub fn new(ctx: &'a mut Context, words: &[u32]) -> Result<Self> {
        return Ok(Self {
            inner: GenericCompiler::new(ctx, sys::spvc_backend::SPVC_BACKEND_MSL, words)?,
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
        let version = (10000 * major) + (100 * minor);
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_VERSION,
            version,
        );
    }

    pub fn texel_buffer_texture_width(mut self, texel_buffer_texture_width: u32) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_TEXEL_BUFFER_TEXTURE_WIDTH,
            texel_buffer_texture_width,
        );
    }

    pub fn aux_buffer_index(mut self, aux_buffer_index: u32) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_AUX_BUFFER_INDEX,
            aux_buffer_index,
        );
    }

    pub fn indirect_params_buffer_index(
        mut self,
        indirect_params_buffer_index: u32,
    ) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_INDIRECT_PARAMS_BUFFER_INDEX,
            indirect_params_buffer_index,
        );
    }

    pub fn shader_output_buffer_index(mut self, shader_output_buffer_index: u32) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_SHADER_OUTPUT_BUFFER_INDEX,
            shader_output_buffer_index,
        );
    }

    pub fn shader_patch_output_buffer_index(
        mut self,
        shader_patch_output_buffer_index: u32,
    ) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_SHADER_PATCH_OUTPUT_BUFFER_INDEX,
            shader_patch_output_buffer_index,
        );
    }

    pub fn shader_tess_factor_output_buffer_index(
        mut self,
        shader_tess_factor_output_buffer_index: u32,
    ) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_SHADER_TESS_FACTOR_OUTPUT_BUFFER_INDEX,
            shader_tess_factor_output_buffer_index,
        );
    }

    pub fn shader_input_workgroup_index(
        mut self,
        shader_input_workgroup_index: u32,
    ) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_SHADER_INPUT_WORKGROUP_INDEX,
            shader_input_workgroup_index,
        );
    }

    pub fn enable_point_size_builtin(mut self, enable_point_size_builtin: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_ENABLE_POINT_SIZE_BUILTIN,
            enable_point_size_builtin,
        );
    }

    pub fn disable_rasterization(mut self, disable_rasterization: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_DISABLE_RASTERIZATION,
            disable_rasterization,
        );
    }

    pub fn capture_output_to_buffer(mut self, capture_output_to_buffer: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_CAPTURE_OUTPUT_TO_BUFFER,
            capture_output_to_buffer,
        );
    }

    pub fn swizzle_texture_samples(mut self, swizzle_texture_samples: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_SWIZZLE_TEXTURE_SAMPLES,
            swizzle_texture_samples,
        );
    }

    pub fn pad_fragment_output_components(
        mut self,
        pad_fragment_output_components: bool,
    ) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_PAD_FRAGMENT_OUTPUT_COMPONENTS,
            pad_fragment_output_components,
        );
    }

    pub fn tess_domain_origin_lower_left(
        mut self,
        tess_domain_origin_lower_left: bool,
    ) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_TESS_DOMAIN_ORIGIN_LOWER_LEFT,
            tess_domain_origin_lower_left,
        );
    }

    pub fn platform(mut self, platform: sys::spvc_msl_platform) -> Result<Self> {
        return self.set_uint(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_PLATFORM,
            platform as u32,
        );
    }

    pub fn argument_buffers(mut self, argument_buffers: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_ARGUMENT_BUFFERS,
            argument_buffers,
        );
    }

    pub fn texture_buffer_native(mut self, texture_buffer_native: bool) -> Result<Self> {
        return self.set_bool(
            sys::spvc_compiler_option::SPVC_COMPILER_OPTION_MSL_TEXTURE_BUFFER_NATIVE,
            texture_buffer_native,
        );
    }
}

impl<'a> From<MslCompiler<'a>> for GenericCompiler<'a> {
    #[inline]
    fn from(value: MslCompiler<'a>) -> Self {
        value.inner
    }
}

impl<'a> Compiler<'a> for MslCompiler<'a> {
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
