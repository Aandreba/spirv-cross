# spirv-cross
High and low level bindigns to the SPIRV-Cross API

## Example
```rust
use spirvcross::{
    bytes_to_words,
    compiler::{glsl::GlslCompiler, Compiler},
    Result,
    Context,
};

fn compile(bytes: &[u8]) -> Result<()> {
    let words = bytes_to_words(bytes).unwrap();

    let mut context = Context::new()?;
    context.set_error_callback(|err| eprintln!("{}", err.to_string_lossy()));

    let compiler = GlslCompiler::new(&mut context, &words)?
        .vulkan_semantics(true)?;

    println!("{}", compiler.compile()?);
    return Ok(());
}
```

## Supported Targets
- Linux
- macOS
- Windows
- WebAssembly (WASI)
    - Compiling to WebAssembly from a Windows machine currently doesn't work.
