use spirv_cross::{
    bytes_to_words,
    compiler::{glsl::GlslCompiler, Compiler},
    error::Result,
    Context,
};

#[cfg(feature = "glsl")]
#[test]
pub fn glsl() -> Result<()> {
    let words = bytes_to_words(include_bytes!("vertex.spv")).unwrap();

    let mut context = Context::new()?;
    #[cfg(feature = "nightly")]
    context.set_error_callback(|err| eprintln!("{}", err.to_string_lossy()));

    let glsl = GlslCompiler::new(&mut context, &words)?;
    println!("{}", glsl.compile()?);

    return Ok(());
}

#[cfg(feature = "hlsl")]
#[test]
pub fn hlsl() -> Result<()> {
    todo!()
}
