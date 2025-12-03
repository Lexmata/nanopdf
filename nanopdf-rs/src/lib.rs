//! NanoPDF - A native Rust PDF library inspired by MuPDF
//!
//! This library provides PDF parsing, rendering, and manipulation capabilities.
//!
//! # FFI Module
//!
//! The `ffi` module provides C-compatible exports that match MuPDF's API.
//! When compiled as a staticlib or cdylib, these functions can be called
//! from C code using the same function signatures as MuPDF.

pub mod fitz;
pub mod pdf;
pub mod ffi;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

