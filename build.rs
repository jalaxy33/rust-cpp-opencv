fn main() {
    // --------- find vcpkg packages ---------
    vcpkg::find_package("opencv").unwrap();

    // --------- find package includes ---------
    let opencv_includes =
        search_package_includes("find_package(OpenCV REQUIRED)", "OpenCV_INCLUDE_DIRS");

    // --------- Build CXX Bridge ---------
    let rust_sources = vec!["src/ffi_bridge.rs"];
    cxx_build::bridges(rust_sources)
        .include("include")
        .includes(opencv_includes)
        .std("c++17")
        .compile("ffi_bridge");

    println!("cargo:rerun-if-changed=src/ffi_bridge.rs");
    println!("cargo:rerun-if-changed=include/ffi_bridge.h");
}

/// Search for include directories of a vcpkg package using CMake
///
/// # Arguments
/// * `find_package_command` - The CMake find_package command, e.g., "find_package(OpenCV REQUIRED)"
/// * `include_dirs_var` - The CMake variable name for include directories, e.g., "OpenCV_INCLUDE_DIRS"
fn search_package_includes(
    find_package_command: &str,
    include_dirs_var: &str,
) -> Vec<std::path::PathBuf> {
    let project_root = std::env::var("CARGO_MANIFEST_DIR")
        .unwrap()
        .replace("\\", "/");

    let cpp_contents = r#"int main() {}"#;

    let cmake_contents = format!(
        r#"
cmake_minimum_required(VERSION 3.15)

set(CMAKE_TOOLCHAIN_FILE "$ENV{{VCPKG_ROOT}}/scripts/buildsystems/vcpkg.cmake")
set(VCPKG_INSTALLED_DIR "$ENV{{VCPKG_ROOT}}/installed")   # use system-wide vcpkg installation
set(VCPKG_MANIFEST_DIR "{}")  # set the manifest directory to the project root

project(search_includes VERSION 0.1.0 LANGUAGES C CXX)

{}
message(STATUS "INCLUDE_DIRS: ${{{}}}")

add_executable(temp temp.cpp)
    "#,
        project_root.as_str(), find_package_command, include_dirs_var
    );

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let temp_build_dir = std::path::Path::new(&out_dir).join("temp");
    std::fs::create_dir_all(&temp_build_dir).unwrap();

    // Write temp.cpp
    let temp_cpp_path = temp_build_dir.join("temp.cpp");
    std::fs::write(&temp_cpp_path, cpp_contents).unwrap();

    // Write CMakeLists.txt
    let cmake_path = temp_build_dir.join("CMakeLists.txt");
    std::fs::write(&cmake_path, cmake_contents).unwrap();

    // Run cmake
    let output = std::process::Command::new("cmake")
        .current_dir(&temp_build_dir)
        .arg(".")
        .output()
        .expect("Failed to run cmake");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let mut include_dirs = Vec::new();

    // Search for INCLUDE_DIRS in both stdout and stderr
    for line in stdout.lines().chain(stderr.lines()) {
        if line.contains("INCLUDE_DIRS:") {
            // Parse the line to extract include directories
            // Format: "-- INCLUDE_DIRS: path1;path2;path3"
            if let Some(dirs_part) = line.split("INCLUDE_DIRS:").nth(1) {
                let dirs = dirs_part.trim();
                // Split by semicolon (CMake list separator)
                for dir in dirs.split(';') {
                    let dir = dir.trim();
                    if !dir.is_empty() {
                        include_dirs.push(std::path::PathBuf::from(dir));
                    }
                }
            }
        }
    }

    // Clean up the temporary build directory
    let _ = std::fs::remove_dir_all(&temp_build_dir);

    include_dirs
}
