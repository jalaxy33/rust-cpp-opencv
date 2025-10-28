//! src/main.rs

use anyhow::{Result, ensure};
use opencv as cv;

use core_rlib::from_cpp;
use core_rlib::*;

fn verify_opencv() -> Result<()> {
    use opencv::prelude::*;

    println!("OpenCV version: {}", cv::core::CV_VERSION);

    // Create a simple mat
    let mat = cv::core::Mat::zeros(3, 3, cv::core::CV_8UC1)?.to_mat()?;
    println!("Created a {}x{} matrix", mat.rows(), mat.cols());

    Ok(())
}

fn try_native_rust(img_path: &str) -> Result<()> {
    use opencv::highgui::*;
    use opencv::imgcodecs::*;
    use opencv::prelude::*;

    println!("Calling native Rust function...");
    ensure!(
        is_path_valid(img_path),
        "Image path is invalid: {}",
        img_path
    );

    let img = imread(img_path, IMREAD_COLOR)?;
    println!("  Image size: {} x {}", img.rows(), img.cols());

    let resized = resize_image_native(&img, img.cols() / 2, img.rows() / 2)?;
    println!(
        "  Resized image size: {} x {}",
        resized.rows(),
        resized.cols()
    );

    imshow("Original Image", &img)?;
    imshow(
        format!("Rust Resized ({} x {})", resized.rows(), resized.cols()).as_str(),
        &resized,
    )?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn try_mix_cpp(img_path: &str) -> Result<()> {
    use opencv::highgui::*;
    use opencv::imgcodecs::*;
    use opencv::prelude::*;

    println!("Calling mixed Rust/C++ function...");
    ensure!(
        is_path_valid(img_path),
        "Image path is invalid: {}",
        img_path
    );

    let img = imread(img_path, IMREAD_COLOR)?;
    println!("  Image size: {} x {}", img.rows(), img.cols());

    let flipped = from_cpp::flip_image(&img, 1)?; // Flip by y-axis
    println!(
        "  Flipped image size: {} x {}",
        flipped.rows(),
        flipped.cols()
    );

    imshow("Original Image", &img)?;
    imshow("C++ Flipped", &flipped)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn try_itk_read() -> Result<()> {
    use opencv::highgui::*;
    use opencv::prelude::*;

    println!("Testing ITK read image...");
    let img_path = "assets/example.jpg";

    ensure!(
        is_path_valid(img_path),
        "Image path is invalid: {}",
        img_path
    );

    println!("  Calling from_cpp::itk_read_image...");
    let img = from_cpp::itk_read_image(img_path)?;
    println!("  Image size: {} x {}", img.rows(), img.cols());

    imshow("ITK Read Image", &img)?;
    wait_key(0)?;
    destroy_all_windows()?;

    Ok(())
}

fn main() -> Result<()> {
    verify_opencv()?;

    let img_path = "assets/example.jpg";
    try_native_rust(img_path)?;
    try_mix_cpp(img_path)?;
    try_itk_read()?;

    Ok(())
}
