use spirv_cross::{bytes_to_words, compiler::glsl::GlslCompiler, error::Result, Context};

#[test]
pub fn glsl() -> Result<()> {
    let words = bytes_to_words(include_bytes!("vertex.spv")).unwrap();

    let mut context = Context::new()?;
    let ir = context.parse_spirv(&words)?;

    let glsl = GlslCompiler::new(&mut context, ir);

    return Ok(());
}
