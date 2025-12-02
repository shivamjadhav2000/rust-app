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



2. Windows-Specific Build Tools (Required for Windows)

If you are compiling on Windows, you will need the Microsoft Visual C++ Build Tools.

Download and install the Build Tools for Visual Studio from Microsoft.

During installation, ensure you select the "Desktop development with C++" workload. This provides the necessary linker and libraries for Rust to build native executables.

🚀 Setup and Running the Application

This guide assumes you have the project structure set up: a top-level directory simple_app containing the Cargo.toml file and a src/main.rs file.

1. Project Files

Ensure your simple_app/Cargo.toml and simple_app/src/main.rs files contain the correct code as provided in the project source.

2. Run in Development Mode

Use the cargo run command from the root of the project directory (simple_app/). This will download dependencies, compile the application in debug mode, and launch the window.

# Navigate to the project root
cd simple_app

# Compile and run
cargo run



3. Create a Release Executable (Packaging)

To create a standalone, optimized executable file that you can share, use the cargo build --release command.

cargo build --release



The final executable will be located in the target/release/ directory.

Operating System

Executable Location (from project root)

Windows

target\release\simple_app.exe

Linux

target/release/simple_app

macOS

target/release/simple_app

🐳 Dockerized Windows Build (Cross-Compilation)

If you wish to build the Windows executable without installing the Rust toolchain or the Microsoft Visual C++ Build Tools on your local machine, you can use Docker to cross-compile the application.

Prerequisites for Docker Build

Docker: Docker Desktop must be installed and running on your system.

Steps to Build and Extract the .exe

Build the Docker Image: This process installs the cross-compilation toolchain and builds the Windows .exe inside the container.

# Run this command in the 'simple_app' directory
docker build -t rust-win-builder .


Create a Temporary Container: Start a container based on the built image to easily copy the executable out.

docker create --name app-extractor rust-win-builder


Extract the Windows Executable: Copy the optimized .exe file from the container's build directory to your local target/release/ folder.

# The path inside the container is /app/target/x86_64-pc-windows-msvc/release/simple_app.exe
docker cp app-extractor:/app/target/x86_64-pc-windows-msvc/release/simple_app.exe target/release/


Cleanup: Remove the temporary container.

docker rm app-extractor


The final, clean Windows executable (simple_app.exe) is now available in your local target/release/ folder and can be run directly on any Windows machine.

💡 Troubleshooting on Linux

If you encounter a Broken pipe (os error 32) or NoGlutinConfigs error on Linux, you may be missing system libraries. Try installing common dependencies like the following (for Debian/Ubuntu based systems):

sudo apt update && sudo apt install -y pkg-config libxkbcommon-dev libwayland-dev libasound-dev libudev
