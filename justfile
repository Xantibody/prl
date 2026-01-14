# Default recipe
default:
    @just --list

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test

# Format code
fmt:
    nix fmt

# Check formatting
fmt-check:
    nix flake check

# Build project
build:
    cargo build

# Build release
release:
    cargo build --release

# Clean build artifacts
clean:
    cargo clean
