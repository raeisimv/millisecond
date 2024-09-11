set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test:
    cargo check
    cargo clippy --workspace --all-targets -- --deny warnings
    cargo fmt --all -- --emit=files
    cargo build
    cargo test --no-fail-fast

build:
    just test
    cargo build --release

publish:
    just build
    cargo publish