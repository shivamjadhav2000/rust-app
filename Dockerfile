 Base image for Rust compilation
FROM rust:latest as builder

# Set up the cross-compilation targets

# 1. Windows: MSVC ABI is required for eframe on Windows
RUN rustup target add x86_64-pc-windows-msvc

# 2. macOS: The standard target for modern Intel/x86_64 Macs
# Note: Building a true .dmg is complex and requires platform-specific tools, 
# but this step generates the executable binary.
RUN rustup target add x86_64-apple-darwin

# Install packages required for macOS cross-compilation from Linux
# We need clang and linker support for the apple-darwin target.
RUN apt-get update && apt-get install -y clang libssl-dev pkg-config

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo files first to cache dependencies efficiently
COPY Cargo.toml Cargo.toml
COPY src src

# Build the release executable for the Windows target
RUN cargo build --release --target x86_64-pc-windows-msvc

# Build the release executable for the macOS target
RUN cargo build --release --target x86_64-apple-darwin

# The final executables are now built inside the container.
# - Windows: /app/target/x86_64-pc-windows-msvc/release/simple_app.exe
# - macOS: /app/target/x86_64-apple-darwin/release/simple_app
