#pragma once

#include <string>
#include <itkImage.h>
#include <itkImageFileReader.h>
#include <itkImageIOFactory.h>

// Register ITK IO factories for common image formats
#include <itkJPEGImageIOFactory.h>
#include <itkPNGImageIOFactory.h>
#include <itkBMPImageIOFactory.h>
#include <itkTIFFImageIOFactory.h>

#include <opencv2/opencv.hpp>

#include "common.h"

namespace itk_utils
{

    // Register common image IO factories
    inline void registerIOFactories()
    {
        static bool registered = false;
        if (!registered)
        {
            itk::JPEGImageIOFactory::RegisterOneFactory();
            itk::PNGImageIOFactory::RegisterOneFactory();
            itk::BMPImageIOFactory::RegisterOneFactory();
            itk::TIFFImageIOFactory::RegisterOneFactory();
            registered = true;
        }
    }

    using PixelType = unsigned char;
    using ImageType = itk::Image<PixelType, 2>;

    inline ImageType::Pointer readImage(const std::string &filename)
    {
        // Ensure IO factories are registered
        registerIOFactories();
        
        using ReaderType = itk::ImageFileReader<ImageType>;
        ReaderType::Pointer reader = ReaderType::New();
        reader->SetFileName(filename);
        try
        {
            reader->Update();
        }
        catch (itk::ExceptionObject &err)
        {
            throw_error("Error reading image: " + std::string(err.GetDescription()));
            return nullptr;
        }
        return reader->GetOutput();
    }

    inline cv::Mat convertToCvMat(itk_utils::ImageType::Pointer itk_image)
    {
        if (!itk_image)
        {
            throw_error("ITK image pointer is null");
            return cv::Mat();
        }

        // 获取图像区域和大小
        ImageType::RegionType region = itk_image->GetLargestPossibleRegion();
        ImageType::SizeType size = region.GetSize();

        // 获取图像的原始数据指针
        PixelType *buffer = itk_image->GetBufferPointer();

        // 创建 OpenCV Mat，注意 ITK 和 OpenCV 的行列顺序
        // ITK: [width, height], OpenCV: [height, width]
        cv::Mat cv_image(size[1], size[0], CV_8UC1, buffer);

        // 返回深拷贝，避免数据生命周期问题
        return cv_image.clone();
    }

    inline cv::Mat readImageAsCvMat(const std::string &filename)
    {
        ImageType::Pointer itk_image = readImage(filename);
        if (!itk_image)
        {
            throw_error("Failed to read image using ITK: " + filename);
            return cv::Mat();
        }
        cv::Mat cv_image = convertToCvMat(itk_image);
        if (cv_image.empty())
        {
            throw_error("Failed to convert ITK image to OpenCV Mat");
            return cv::Mat();
        }
        return cv_image;
    }

} // namespace itk_utils





