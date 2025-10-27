//! include/common.h
//! Common utilities and error handling
#pragma once

#include <iostream>
#include <filesystem>
#include <stdexcept>
#include <string>

inline void throw_error(const std::string &msg)
{
    std::cerr << "Error: " << msg << std::endl;
    throw std::runtime_error(msg);
}

inline void assert_file_exists(const std::filesystem::path &path)
{
    if (!std::filesystem::exists(path))
    {
        throw_error("File does not exist: " + path.string());
    }
}