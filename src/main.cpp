#include <iostream>
#include <string>
#include <filesystem>
#include <opencv2/opencv.hpp>

#include "common.h"
#include "example.h"
#include "from_rust.h"

#include "itk_example.h"

using namespace std;

void verify_opencv()
{
    std::cout << "OpenCV version : " << CV_VERSION << std::endl;

    // Create a 3x3 matrix
    cv::Mat mat = cv::Mat::zeros(3, 3, CV_8UC1);
    std::cout << "Created a " << mat.rows << "x" << mat.cols << " matrix" << std::endl;
}

void try_native_cpp(const string &img_path)
{
    cout << "Calling native C++ function..." << endl;

    assert_file_exists(img_path);

    cv::Mat img = cv::imread(img_path);
    if (img.empty())
    {
        throw_error("Failed to load image: " + img_path);
    }

    cv::Mat flipped = flip_image_native(img, 1); // Flip around y-axis
    if (flipped.empty())
    {
        throw_error("Failed to flip image");
    }

    cout << " Image flipped successfully. Original size: " << img.size() << ", Flipped size: " << flipped.size() << endl;

    cv::imshow("Original Image", img);
    cv::imshow("C++ Flipped", flipped);
    cv::waitKey(0);
    cv::destroyAllWindows();
}

void try_mix_rust(const string &img_path)
{
    cout << "Calling Rust function via C++..." << endl;

    assert_file_exists(img_path);

    cv::Mat img = cv::imread(img_path);
    if (img.empty())
    {
        throw_error("Failed to load image: " + img_path);
    }

    cv::Mat resized = from_rust::resize_image(img, img.cols / 2, img.rows / 2);
    if (resized.empty())
    {
        throw_error("Failed to resize image via Rust");
    }

    cout << " Image resized successfully via Rust. Original size: " << img.size() << ", Resized size: " << resized.size() << endl;

    cv::imshow("Original Image", img);
    cv::imshow("Rust Resized (" + std::to_string(resized.cols) + "x" + std::to_string(resized.rows) + ")", resized);
    cv::waitKey(0);
    cv::destroyAllWindows();
}

void try_itk(const std::string &img_path)
{
    std::cout << "Calling ITK function via C++..." << std::endl;

    // ----------- Read image using ITK -----------

    itk_utils::ImageType::Pointer image = itk_utils::readImage(img_path);
    if (!image)
    {
        throw_error("Failed to read image using ITK: " + img_path);
    }
    std::cout << " ITK Image read successfully. Size: " << image->GetLargestPossibleRegion().GetSize() << std::endl;

    // ----------- Convert ITK image to OpenCV Mat -----------

    cv::Mat cv_image = itk_utils::convertToCvMat(image);

    if (cv_image.empty())
    {
        throw_error("Failed to convert ITK image to OpenCV Mat");
    }
    std::cout << " Converted ITK image to OpenCV Mat successfully. Size: " << cv_image.size() << std::endl;

    // ----------- All in one: Read and convert -----------

    cv::Mat cv_image_all_in_one = itk_utils::readImageAsCvMat(img_path);
    if (cv_image_all_in_one.empty())
    {
        throw_error("Failed to read and convert image using ITK");
    }
    std::cout << " Read and converted image using ITK successfully. Size: " << cv_image_all_in_one.size() << std::endl;

    // ----------- Display the image using OpenCV -----------

    cv::imshow("ITK Converted Image", cv_image);
    cv::imshow("ITK Read-Converted Image", cv_image_all_in_one);
    cv::waitKey(0);
    cv::destroyAllWindows();
}

int main()
{
    verify_opencv();

    string project_root = PROJECT_ROOT;
    string img_path = filesystem::path(project_root).append("assets/example.jpg").string();

    try_native_cpp(img_path);
    try_mix_rust(img_path);

    try_itk(img_path);

    return 0;
}
