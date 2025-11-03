//! from_cpp.rs
//!
//! Interface to use functions from C++

use anyhow::Result;
use opencv as cv;
use opencv::prelude::*;

use super::{bridge::ffi, cv_conversion};

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use opencv as cv;

    #[test]
    fn test_flip_image() {
        let img = create_dummy_image(100, 100).unwrap();
        let flipped = flip_image(&img, 1).unwrap(); // Flip by y-axis
        assert_eq!(flipped.rows(), img.rows());
        assert_eq!(flipped.cols(), img.cols());
    }

    // ********** Helper Functions for Tests **********

    fn create_dummy_image(width: i32, height: i32) -> Result<cv::core::Mat> {
        use opencv::prelude::*;

        let mat = cv::core::Mat::zeros(height, width, cv::core::CV_8UC3)?.to_mat()?;
        Ok(mat)
    }
}
