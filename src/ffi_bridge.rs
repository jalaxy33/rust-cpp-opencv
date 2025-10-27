//! src/ffi_bridge.rs
//! FFI Bridge between Rust and C++

use anyhow::Result;
use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("ffi_bridge.h");

        /// ******** Shared Types ********
        type CMat;

        /// ******** Conversion Functions ********
        fn rust_mat_to_cpp_ref(rust_mat_ptr_addr: usize) -> *const CMat;
        fn rust_to_cpp_safe(rust_mat_ptr_addr: usize) -> UniquePtr<CMat>;
        fn cpp_to_rust_safe(cpp_mat: &CMat) -> usize;

        /// ******** Functions from C++ ********
        fn _flip_image_cpp(input_mat: &CMat, flip_code: i32) -> UniquePtr<CMat>;
    }

    extern "Rust" {
        fn _resize_image_rust(img: &CMat, width: i32, height: i32) -> Result<UniquePtr<CMat>>;
    }
}

// OpenCV Conversion utilities between Rust and C++
#[allow(dead_code)]
pub mod cv_conversion {
    use super::*;

    use anyhow::Result;
    use cxx::UniquePtr;
    use opencv as cv;
    use opencv::prelude::*;

    // ************ Core Conversion Functions ************

    /// Rust Mat -> C++ Mat (safe copy)
    pub fn safe_convert_rust_to_cpp(rust_mat: &cv::core::Mat) -> Result<UniquePtr<ffi::CMat>> {
        if rust_mat.empty() {
            anyhow::bail!("Cannot convert empty Mat");
        }

        let ptr_addr = rust_mat.as_raw_Mat() as usize;
        Ok(super::ffi::rust_to_cpp_safe(ptr_addr))
    }

    /// C++ Mat -> Rust Mat (safe copy)
    pub fn safe_convert_cpp_to_rust(cpp_mat: &ffi::CMat) -> Result<cv::core::Mat> {
        let ptr_addr = super::ffi::cpp_to_rust_safe(cpp_mat);
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
        let cpp_ref_ptr = super::ffi::rust_mat_to_cpp_ref(ptr_addr);
        Ok(unsafe { &*cpp_ref_ptr })
    }

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
}

/// Modules exposing functions that use C++ implementations
#[allow(dead_code)]
pub mod from_cpp {
    use super::*;
    use anyhow::Result;
    use opencv as cv;
    use opencv::prelude::*;

    pub fn flip_image(img: &cv::core::Mat, flip_code: i32) -> Result<cv::core::Mat> {
        if img.empty() {
            anyhow::bail!("Cannot process empty Mat");
        }

        unsafe {
            // Rust Mat -> C++ Mat (Zero-copy conversion)
            let c_img = cv_conversion::zero_copy_rust_to_cpp_ref(img)?;

            // Call C++ function
            let flipped = ffi::_flip_image_cpp(c_img, flip_code);

            // C++ Mat -> Rust Mat
            let result = cv_conversion::safe_convert_cpp_to_rust(&flipped)?;

            Ok(result)
        }
    }
}

// --------------- Rust -> C++ Functions ---------------

fn _resize_image_rust(img: &ffi::CMat, width: i32, height: i32) -> Result<UniquePtr<ffi::CMat>> {
    // Convert C++ Mat to Rust Mat
    let rust_img = cv_conversion::safe_convert_cpp_to_rust(img)?;

    // Resize using OpenCV in Rust
    let resized = crate::resize_image_native(&rust_img, width, height)?;

    // Convert resized Rust Mat back to C++ Mat
    let cpp_resized = cv_conversion::safe_convert_rust_to_cpp(&resized)?;

    Ok(cpp_resized)
}

// ---------------- Unit Test ----------------

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use opencv as cv;
    use opencv::prelude::*;

    #[test]
    fn test_flip_image() {
        let img = create_dummy_image(100, 100).unwrap();
        let flipped = from_cpp::flip_image(&img, 1).unwrap(); // Flip by y-axis
        assert_eq!(flipped.rows(), img.rows());
        assert_eq!(flipped.cols(), img.cols());
    }

    #[test]
    fn test_resize_image_rust() {
        let img = create_dummy_image(100, 100).unwrap();

        // Convert Rust Mat to C++ Mat
        let cpp_img = cv_conversion::safe_convert_rust_to_cpp(&img).unwrap();

        // Call Rust resize function via FFI
        let resized_cpp = _resize_image_rust(&cpp_img, 50, 50).unwrap();

        // Convert resized C++ Mat back to Rust Mat
        let resized_rust = cv_conversion::safe_convert_cpp_to_rust(&resized_cpp).unwrap();

        assert_eq!(resized_rust.rows(), 50);
        assert_eq!(resized_rust.cols(), 50);
    }

    // ********** Helper Functions for Tests **********

    fn create_dummy_image(width: i32, height: i32) -> Result<cv::core::Mat> {
        let mat = cv::core::Mat::zeros(height, width, cv::core::CV_8UC3)?.to_mat()?;
        Ok(mat)
    }
}
