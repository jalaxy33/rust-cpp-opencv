# Rust-C++ OpenCV Interoperability demo

This project demonstrates how to interoperate between Rust and C++ using OpenCV for image processing tasks. It includes examples of reading, resizing, and displaying images using both C++ and Rust functions.


## Prerequisites

- **C++ compiler**: g++/clang++/MSVC
- **[CMake](https://cmake.org/download/) build tools**
- **[Rust toolchain](https://www.rust-lang.org/tools/install)**
- **[vcpkg](https://vcpkg.io/en/index.html)**: C/C++ package manager.


## Usage

### 1. Running Rust project

1. Build the Rust project:

    ```bash
    cargo build   # debug build (or `cargo r`)
    cargo build --release  # release build (or `cargo r -r`)
    ```

2. Run the Rust binary:

    ```bash
    cargo run   # debug build (or `cargo r`)
    cargo run --release  # release build (or `cargo r -r`)
    ```

3. Run all Rust tests:

    ```bash
    cargo test  # debug build (or `cargo t`)
    cargo test --release  # release build (or `cargo t -r`)
    ```


### 2. Running C++ project

1. Configure the C++ project using CMake:

    ```bash
    cmake -B build
    ```

2. Build the C++ project:

    ```bash
    cmake --build build   # debug build
    cmake --build build --config Release # release build
    ```


3. Run the C++ executable:

    ```bash
    # Debug build
    ./build/Debug/demo.exe  # On Windows
    ./build/Debug/demo  # On Linux/macOS

    # Release build
    ./build/Release/demo.exe  # On Windows
    ./build/Release/demo  # On Linux/macOS
    ```

4. Run all tests:

    ```bash
    ctest --test-dir build -C Debug -j24  # for Debug build
    ctest --test-dir build -C Release -j24  # for Release build
    ```