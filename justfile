export RUST_BACKTRACE := "1"

submodule:
    rm -rf SPIRV-Cross
    git submodule update --init --recursive

doc:
    cargo +nightly rustdoc --open --all-features -- --cfg docsrs

test:
    cargo +nightly test --all --all-features -- --nocapture

test-wasm:
    cargo +nightly wasi test --verbose --all-features
