# Base image for Rust compilation
FROM rust:latest as builder

# Set up the Windows cross-compilation target (MSVC ABI is required for eframe on Windows)
RUN rustup target add x86_64-pc-windows-msvc

# Set the working directory inside the container
WORKDIR /app

# Copy the Cargo files first to cache dependencies efficiently
COPY Cargo.toml Cargo.toml
COPY src src

# Build the release executable for the Windows target
# This ensures a lightweight and performant final binary.
RUN cargo build --release --target x86_64-pc-windows-msvc

# --- SECOND STAGE: Create a small final image (optional, but good practice) ---
# Since we are just extracting the binary, we don't strictly need a second stage, 
# but this is how you'd structure a clean container.
# FROM scratch
# WORKDIR /

# The final executable is not copied into the final Docker image (scratch) because
# the goal is to extract the .exe to the Windows host, not run it inside Docker.
# The user will run the 'docker cp' command below to get the result.
