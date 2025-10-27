#pragma once

#include <opencv2/opencv.hpp>

inline cv::Mat flip_image_native(const cv::Mat &input_image, int flip_code)
{
    cv::Mat flipped_image;
    cv::flip(input_image, flipped_image, flip_code);
    return flipped_image;
}
