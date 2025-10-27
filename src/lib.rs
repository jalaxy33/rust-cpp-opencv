//! src/lib.rs

use anyhow::Result;
use opencv as cv;

mod ffi_bridge;
pub use ffi_bridge::from_cpp;

// ---------------- Rust Native Functions ----------------

pub fn is_path_valid(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

pub fn resize_image_native(img: &cv::core::Mat, width: i32, height: i32) -> Result<cv::core::Mat> {
    use opencv::imgproc::*;

    let mut resized = cv::core::Mat::default();
    let size = cv::core::Size::new(width, height);
    resize(img, &mut resized, size, 0.0, 0.0, INTER_LINEAR)?;
    Ok(resized)
}

// ---------------- Unit Test ----------------

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use opencv as cv;
    use opencv::prelude::*;

    #[test]
    fn test_is_path_valid() {
        assert!(is_path_valid("src/lib.rs")); // assuming this file exists
        assert!(!is_path_valid("non_existent_file.txt"));
    }

    #[test]
    fn test_resize_image_native() {
        let img = create_dummy_image(100, 100).unwrap();
        let resized = resize_image_native(&img, 50, 50).unwrap();
        assert_eq!(resized.rows(), 50);
        assert_eq!(resized.cols(), 50);
    }

    // ********** Helper Functions for Tests **********

    fn create_dummy_image(width: i32, height: i32) -> Result<cv::core::Mat> {
        let mat = cv::core::Mat::zeros(height, width, cv::core::CV_8UC3)?.to_mat()?;
        Ok(mat)
    }
}
