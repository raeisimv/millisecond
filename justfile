set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
set shell := ["bash", "-cu"]

test:
    cargo check
    cargo clippy --workspace --all-targets -- --deny warnings
    cargo fmt --all -- --emit=files
    cargo build
    cargo test --no-fail-fast

build:
    just setup
    just test
    cargo build --release

publish:
    just build
    cargo publish

setup:
    #install pre-commit hook
    cp scripts/hooks/pre-commit.sh .git/hooks/pre-commit
