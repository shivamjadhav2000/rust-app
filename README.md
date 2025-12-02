Simple Rust Desktop Dashboard

This is a minimal, cross-platform desktop application built using Rust and the eframe GUI library (which utilizes egui). It serves as a starter template demonstrating basic GUI elements, including a live clock and a functional To-Do list.

✨ Features

Live Digital Clock: Displays the current time and date, updated every frame.

"Hello World" Greeter: An input field to customize the greeting.

Simple To-Do List: Allows users to add and remove tasks instantly.

🛠️ Prerequisites

To build and run this application, you must have the Rust toolchain installed.

1. Install Rust (Required)

The recommended way to install Rust is using rustup.

Windows, macOS, and Linux:
Follow the instructions at the official rustup website or run the following command in your terminal:

curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh


2. Required Build Tools for Manual Cross-Compilation

To build an executable for a platform different from your host OS, you must have the target toolchain and linker installed locally.

Host OS

Target OS

Required Toolchain/Package

Installation Step (Example for Debian/Ubuntu)

Linux/WSL

Windows (.exe)

MinGW Cross-Compiler

sudo apt install gcc-mingw-w64-x86-64

Windows

Windows (.exe)

MSVC Build Tools

Install "Desktop development with C++" workload from Visual Studio Build Tools.

macOS

macOS (.app)

Xcode Command Line Tools

xcode-select --install

Note: Building macOS executables (targeting macOS) requires running the build on a native macOS machine with Xcode installed.

🚀 Setup and Running the Application

This guide assumes you have the project structure set up: a top-level directory simple_app containing the Cargo.toml file and a src/main.rs file.

1. Run in Development Mode

Use the cargo run command from the root of the project directory (simple_app/). This will download dependencies, compile the application in debug mode, and launch the window.

# Navigate to the project root
cd simple_app

# Compile and run (Debug build)
cargo run


2. Create a Release Executable

To create a standalone, optimized executable file, use the cargo build --release command, optionally specifying a target platform.

Target Operating System

Target Command

Executable Location (from project root)

Linux (Native)

cargo build --release

target/release/simple_app

Windows (Cross-compiled)

cargo build --release --target x86_64-pc-windows-gnu

target/x86_64-pc-windows-gnu/release/simple_app.exe

macOS (Native build on Mac)

cargo build --release

target/release/simple_app

💡 Troubleshooting on Linux

If you encounter a Broken pipe (os error 32) or NoGlutinConfigs error on Linux, you may be missing system libraries. Try installing common dependencies like the following (for Debian/Ubuntu based systems):

sudo apt update && sudo apt install -y pkg-config libxkbcommon-dev libwayland-dev libasound-dev libude