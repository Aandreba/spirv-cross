export RUST_BACKTRACE := "1"

submodule:
    git submodule update --init --recursive

doc:
    cargo +nightly rustdoc --open --all-features -- --cfg docsrs

test:
    cargo +nightly test --all --all-features -- --nocapture

test-wasm:
    cargo +nightly build --tests --all-features --target wasm32-wasi
