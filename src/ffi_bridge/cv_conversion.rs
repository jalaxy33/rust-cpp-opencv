//! cv_conversion.rs
//! 
//! OpenCV Conversion utilities between Rust and C++

use anyhow::Result;
use cxx::UniquePtr;
use opencv as cv;
use opencv::prelude::*;

use super::bridge::ffi;

// ************ Internal Helper Functions ************

/// Create Rust Mat from C++ pointer address
unsafe fn from_ptr_addr(ptr_addr: usize) -> cv::core::Mat {
    use cv::core::Mat;

    if ptr_addr == 0 {
        return Mat::default();
    }

    let cv_mat_ptr = ptr_addr as *mut std::ffi::c_void;
    unsafe { Mat::from_raw(cv_mat_ptr) }
}

// ************ Core Conversion Functions ************

/// Rust Mat -> C++ Mat (safe copy)
pub fn safe_convert_rust_to_cpp(rust_mat: &cv::core::Mat) -> Result<UniquePtr<ffi::CMat>> {
    if rust_mat.empty() {
        anyhow::bail!("Cannot convert empty Mat");
    }

    let ptr_addr = rust_mat.as_raw_Mat() as usize;
    Ok(ffi::rust_to_cpp_safe(ptr_addr))
}

/// C++ Mat -> Rust Mat (safe copy)
pub fn safe_convert_cpp_to_rust(cpp_mat: &ffi::CMat) -> Result<cv::core::Mat> {
    let ptr_addr = ffi::cpp_to_rust_safe(cpp_mat);
    if ptr_addr == 0 {
        anyhow::bail!("Failed to convert C++ Mat: null pointer returned");
    }

    unsafe { Ok(from_ptr_addr(ptr_addr)) }
}

/// Rust Mat -> C++ Mat reference (zero-copy)
///
/// ⚠️ Must be called within unsafe block, ensure Rust Mat lifetime covers entire usage period
pub unsafe fn zero_copy_rust_to_cpp_ref(rust_mat: &cv::core::Mat) -> Result<&ffi::CMat> {
    if rust_mat.empty() {
        anyhow::bail!("Cannot convert empty Mat");
    }

    let ptr_addr = rust_mat.as_raw_Mat() as usize;
    let cpp_ref_ptr = ffi::rust_mat_to_cpp_ref(ptr_addr);
    Ok(unsafe { &*cpp_ref_ptr })
}