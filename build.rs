use std::path::PathBuf;
use std::{env, vec};

fn main() {
    // --------- Setup vcpkg packages ---------
    let opencv_triplet_dir = setup_vcpkg_package("opencv", &["contrib", "nonfree", "world"]);
    let opencv_include = opencv_triplet_dir
        .as_ref()
        .map(|dir| dir.join("include/opencv4"))
        .expect("Failed to get OpenCV include directory");

    // --------- Build CXX Bridge ---------
    let rust_sources = vec!["src/ffi_bridge.rs"];
    cxx_build::bridges(rust_sources)
        .include("include")
        .include(opencv_include)
        .std("c++17")
        .compile("ffi_bridge");

    println!("cargo:rerun-if-changed=src/ffi_bridge.rs");
    println!("cargo:rerun-if-changed=include/ffi_bridge.h");
}

/// setup vcpkg package linkage
fn setup_vcpkg_package(package_name: &str, features: &[&str]) -> Option<PathBuf> {
    // Get VCPKG_ROOT environment variable
    let vcpkg_root = match env::var("VCPKG_ROOT") {
        Ok(root) => PathBuf::from(root),
        Err(_) => {
            println!("cargo:warning=VCPKG_ROOT environment variable not set");
            return None;
        }
    };

    // Get target triplet
    let triplet = get_target_triplet();
    let installed_path = vcpkg_root.join("installed").join(&triplet);

    // Check if the package is already installed
    if !installed_path.exists() {
        println!(
            "cargo:warning=Package {} not found for triplet {}, installing...",
            package_name, triplet
        );

        // Build vcpkg install command
        let mut install_args = vec!["install".to_string()];

        // Add package name with features
        if features.is_empty() {
            install_args.push(package_name.to_string());
        } else {
            let features_str = features.join(",");
            install_args.push(format!("{}[{}]", package_name, features_str));
        }

        // Add triplet
        install_args.push(format!("--triplet={}", triplet));
        install_args.push("--classic".to_string());

        // Execute vcpkg install command
        println!("cargo:warning=Running: vcpkg {:?}", install_args);

        let output = std::process::Command::new("vcpkg")
            .args(&install_args)
            .output();

        match output {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        "cargo:warning=Failed to install {}: {}",
                        package_name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                    return None;
                }
                println!(
                    "cargo:warning=Successfully installed {} for triplet {}",
                    package_name, triplet
                );
            }
            Err(e) => {
                println!("cargo:warning=Failed to execute vcpkg: {}", e);
                return None;
            }
        }

        // Check again if installation was successful
        if !installed_path.exists() {
            println!("cargo:warning=Installation completed but package directory not found");
            return None;
        }
    } else {
        println!(
            "cargo:warning=Found existing installation for {} with triplet {}",
            package_name, triplet
        );
    }

    // Setup library search paths
    let lib_path = installed_path.join("lib");
    if lib_path.exists() {
        println!("cargo:rustc-link-search=native={}", lib_path.display());
    }

    // Handle DLL paths for Windows dynamic linking
    if cfg!(target_os = "windows") && !triplet.ends_with("-static") {
        let bin_path = installed_path.join("bin");
        if bin_path.exists() {
            println!("cargo:rustc-link-search=native={}", bin_path.display());

            // Add bin path to PATH environment variable for build process
            if let Ok(path) = env::var("PATH") {
                let new_path = format!("{};{}", bin_path.display(), path);
                println!("cargo:rustc-env=PATH={}", new_path);
            }
        }
    }

    // Return the triplet directory path
    Some(installed_path)
}

fn get_target_triplet() -> String {
    let arch = if cfg!(target_arch = "x86_64") {
        "x64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "aarch64") {
        "arm64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else {
        "unknown"
    };

    if cfg!(target_os = "windows") {
        format!("{}-windows", arch)
    } else if cfg!(target_os = "linux") {
        format!("{}-linux", arch)
    } else if cfg!(target_os = "macos") {
        format!("{}-osx", arch)
    } else {
        "unknown-unknown".to_string()
    }
}
