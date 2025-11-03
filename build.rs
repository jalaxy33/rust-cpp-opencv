use anyhow::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    // ------ find c++ libraries ------
    vcpkg::find_package("opencv")?;
    vcpkg::find_package("itk")?;

    // ------ find package includes ------
    let opencv_includes =
        search_package_includes("find_package(OpenCV REQUIRED)", "OpenCV_INCLUDE_DIRS")?;
    let itk_includes =
        search_package_includes("find_package(ITK CONFIG REQUIRED)", "ITK_INCLUDE_DIRS")?;

    println!(
        "cargo:warning=Found OpenCV include dirs: {:?}",
        opencv_includes
    );
    println!(
        "cargo:warning=Found ITK include dirs: {:?}",
        itk_includes
    );
    
    // --------- Build CXX Bridge ---------
    let rust_sources = vec!["src/ffi_bridge.rs"];
    
    cxx_build::bridges(rust_sources)
        .include("include")
        .includes(opencv_includes)
        .includes(itk_includes)
        .std("c++17")
        .try_compile("ffi_bridge")?;

    // ------ Link Windows system libraries ------
    // Required for ITK Windows API dependencies
    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=Advapi32");
        println!("cargo:rustc-link-lib=Shell32");
    }

    // ------ Set rerun triggers ------
    // Avoid unnecessary recompilation
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/ffi_bridge.rs");
    println!("cargo:rerun-if-changed=include/ffi_bridge.h");
    
    Ok(())
}

/// Search for include directories of a vcpkg package using CMake
///
/// # Arguments
/// * `find_package_command` - The CMake find_package command, e.g., "find_package(OpenCV REQUIRED)"
/// * `include_dirs_var` - The CMake variable name for include directories, e.g., "OpenCV_INCLUDE_DIRS"
pub fn search_package_includes(
    find_package_command: &str,
    include_dirs_var: &str,
) -> Result<Vec<PathBuf>> {
    use cargo_metadata::MetadataCommand;
    use regex::Regex;
    use std::collections::HashMap;

    // ----- Get paths ------
    let metadata = MetadataCommand::new().exec()?;
    let workspace_root = metadata.workspace_root;
    let target_dir = metadata.target_directory;

    // ----- Prepare CMake script -----
    let cpp_contents = "int main() {}";
    let cmake_contents = format!(
        r#"
cmake_minimum_required(VERSION 3.15)

set(CMAKE_TOOLCHAIN_FILE "$ENV{{VCPKG_ROOT}}/scripts/buildsystems/vcpkg.cmake")
set(VCPKG_INSTALLED_DIR "$ENV{{VCPKG_ROOT}}/installed")   # use system-wide vcpkg installation
set(VCPKG_MANIFEST_DIR "{manifest_dir}")  # set the manifest directory to the project root

project(search_includes VERSION 0.1.0 LANGUAGES C CXX)

{find_command}
message(STATUS "INCLUDE_DIRS: ${{{include_var}}}")
        "#,
        manifest_dir = workspace_root.to_string().replace("\\", "/"),
        find_command = find_package_command,
        include_var = include_dirs_var,
    );

    // ----- Find include directories -----

    let record_file = target_dir.join("package_includes.json");

    // load records if exists
    let mut records: HashMap<String, Vec<PathBuf>> = HashMap::new();
    if record_file.exists() {
        let record_content =
            std::fs::read_to_string(&record_file).expect("Failed to read package_includes.json");
        records =
            serde_json::from_str(&record_content).expect("failed to parse package_includes.json");
    }

    // return if recorded
    if records.contains_key(include_dirs_var) {
        let include_dirs = records.get(include_dirs_var).unwrap().clone();
        return Ok(include_dirs);
    }

    // Create temp build directory and files
    let temp_build_dir = target_dir.join("temp");
    let temp_cpp_path = temp_build_dir.join("temp.cpp");
    let temp_cmake_path = temp_build_dir.join("CMakeLists.txt");

    if temp_build_dir.exists() {
        std::fs::remove_dir_all(&temp_build_dir)?;
    }

    std::fs::create_dir_all(&temp_build_dir)?;
    std::fs::write(&temp_cpp_path, cpp_contents)?;
    std::fs::write(&temp_cmake_path, cmake_contents)?;

    // Run CMake to configure the project
    let cmake_output = std::process::Command::new("cmake")
        .current_dir(&temp_build_dir)
        .arg(".")
        .output()
        .expect("Failed to run cmake");

    // Parse CMake output to find include directories
    let stdout = String::from_utf8_lossy(&cmake_output.stdout);
    let stderr = String::from_utf8_lossy(&cmake_output.stderr);

    let output_text = format!("{}\n{}", stdout, stderr);
    let re = Regex::new(r"INCLUDE_DIRS:\s*(.*)").unwrap();
    let mut include_dirs = Vec::new();

    if let Some(caps) = re.captures(&output_text) {
        let dirs = caps.get(1).unwrap().as_str();
        for dir in dirs.split(';') {
            let dir = dir.trim();
            if !dir.is_empty() {
                include_dirs.push(PathBuf::from(dir));
            }
        }
    }

    // update record file
    records.insert(include_dirs_var.to_string(), include_dirs.clone());

    // Save record
    let record_content =
        serde_json::to_string_pretty(&records).expect("Failed to serialize package includes");

    std::fs::write(&record_file, record_content)
        .expect(format!("Failed to save {}", record_file.to_string()).as_str());

    Ok(include_dirs)
}
