set windows-shell := ["powershell.exe", "-c"]
export RUST_BACKTRACE := "1"

submodule:
    git submodule update --init --recursive

doc:
    cargo +nightly rustdoc --open --all-features -- --cfg docsrs

test:
    cargo +nightly test --all --all-features -- --nocapture

test-wasm:
    cargo +nightly wasi test --verbose --all-features
