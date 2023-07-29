# spirv-cross
High and low level bindigns to the SPIRV-Cross API

## Example
```rust
use spirv_cross::{
    bytes_to_words,
    compiler::{glsl::GlslCompiler, Compiler},
    error::Result,
    Context,
};

pub fn main() -> Result<()> {
    let words = bytes_to_words(include_bytes!("vertex.spv")).unwrap();

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
