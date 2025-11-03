//! to_cpp.rs
//! 
//! Interface to export Rust functions to C++

use anyhow::Result;
use cxx::UniquePtr;

use super::{bridge::ffi, cv_conversion};


pub fn _resize_image_rust(img: &ffi::CMat, width: i32, height: i32) -> Result<UniquePtr<ffi::CMat>> {
    // Convert C++ Mat to Rust Mat
    let rust_img = cv_conversion::safe_convert_cpp_to_rust(img)?;

    // Resize using OpenCV in Rust
    let resized = crate::resize_image_native(&rust_img, width, height)?;

    // Convert resized Rust Mat back to C++ Mat
    let cpp_resized = cv_conversion::safe_convert_rust_to_cpp(&resized)?;

    Ok(cpp_resized)
}


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use opencv as cv;
    
    #[test]
    fn test_resize_image_rust() {
        use opencv::prelude::*;

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
        use opencv::prelude::*;

        let mat = cv::core::Mat::zeros(height, width, cv::core::CV_8UC3)?.to_mat()?;
        Ok(mat)
    }
}