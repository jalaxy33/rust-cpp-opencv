//! bridge.rs
//! 
//! CXX Bridge between Rust and C++

use super::to_cpp::*;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("ffi_bridge.h");

        /// ******** Shared Types ********
        type CMat;

        /// ******** Conversion Functions ********
        pub fn rust_mat_to_cpp_ref(rust_mat_ptr_addr: usize) -> *const CMat;
        pub fn rust_to_cpp_safe(rust_mat_ptr_addr: usize) -> UniquePtr<CMat>;
        pub fn cpp_to_rust_safe(cpp_mat: &CMat) -> usize;

        /// ******** Rust -> C++ Functions ********
        pub fn _flip_image_cpp(img: &CMat, flip_code: i32) -> UniquePtr<CMat>;
    }

    extern "Rust" {
        fn _resize_image_rust(img: &CMat, width: i32, height: i32) -> Result<UniquePtr<CMat>>;
    }
}