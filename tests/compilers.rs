use spirv_cross::{bytes_to_words, compiler::Compiler, Context, Result};

#[cfg(feature = "glsl")]
#[test]
pub fn glsl() -> Result<()> {
    use spirv_cross::compiler::glsl::{EsOptions, GlslCompiler};
    let words = bytes_to_words(include_bytes!("vertex.spv")).unwrap();

    let mut context = Context::new()?;
    #[cfg(feature = "nightly")]
    context.set_error_callback(|err| eprintln!("{}", err.to_string_lossy()));

    let glsl = GlslCompiler::new(&mut context, &words)?
        .es(Some(EsOptions::default()))?
        .vulkan_semantics(true)?;

    println!("{}", glsl.compile()?);

    return Ok(());
}

#[cfg(feature = "hlsl")]
#[test]
pub fn hlsl() -> Result<()> {
    use spirv_cross::compiler::hlsl::HlslCompiler;
    let words = bytes_to_words(include_bytes!("vertex.spv")).unwrap();

    let mut context = Context::new()?;
    #[cfg(feature = "nightly")]
    context.set_error_callback(|err| eprintln!("{}", err.to_string_lossy()));

    let hlsl = HlslCompiler::new(&mut context, &words)?;
    println!("{}", hlsl.compile()?);

    return Ok(());
}
