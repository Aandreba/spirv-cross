export RUST_BACKTRACE := "1"

doc:
    cargo +nightly rustdoc --open --all-features -- --cfg docsrs

test:
    cargo +nightly test --all --all-features -- --nocapture
