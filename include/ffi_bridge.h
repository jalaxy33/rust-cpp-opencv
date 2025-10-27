//! ffi_bridge.h
//! FFI Bridge between C++ and Rust
#pragma once
#include <memory>
#include <opencv2/opencv.hpp>

#include "common.h"
#include "example.h"

// ------------- Shared Types -------------

using CMat = cv::Mat;

// ----------- Conversion Functions -----------

/// Convert Rust Mat pointer to C++ Mat reference (zero-copy)
inline const CMat *rust_mat_to_cpp_ref(uintptr_t rust_mat_ptr_addr)
{
    if (rust_mat_ptr_addr == 0)
    {
        throw_error("rust_mat_to_cpp_ref: null pointer address provided");
    }

    const cv::Mat *cv_mat = reinterpret_cast<const cv::Mat *>(rust_mat_ptr_addr);

    if (cv_mat->empty())
    {
        throw_error("rust_mat_to_cpp_ref: provided Mat is empty");
    }

    return cv_mat;
}

/// Convert Rust Mat pointer to C++ Mat copy (safe)
inline std::unique_ptr<CMat> rust_to_cpp_safe(uintptr_t rust_mat_ptr_addr)
{
    if (rust_mat_ptr_addr == 0)
    {
        throw_error("rust_to_cpp_safe: null pointer address provided");
    }

    const cv::Mat *source_mat = reinterpret_cast<const cv::Mat *>(rust_mat_ptr_addr);

    if (source_mat->empty())
    {
        throw_error("rust_to_cpp_safe: source Mat is empty");
    }

    auto cpp_mat = std::make_unique<cv::Mat>(*source_mat);

    return cpp_mat;
}

/// Convert C++ Mat to Rust pointer address (creates copy)
inline uintptr_t cpp_to_rust_safe(const CMat &cpp_mat)
{
    if (cpp_mat.empty())
    {
        throw_error("cpp_to_rust_safe: source Mat is empty");
    }

    cv::Mat *new_mat = new cv::Mat(cpp_mat);

    return reinterpret_cast<uintptr_t>(new_mat);
}

// ----------- C++ -> Rust -----------

inline std::unique_ptr<CMat> _flip_image_cpp(const CMat &input_image, int flip_code)
{
    cv::Mat flipped_image = flip_image_native(input_image, flip_code);
    if (flipped_image.empty())
    {
        throw_error("flip_image_cpp: flipped image is empty");
    }
    return std::make_unique<CMat>(flipped_image);
}
