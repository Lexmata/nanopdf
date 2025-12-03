# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release preparation

## [0.1.0] - 2025-01-XX

### Added

#### Core Library (`fitz` module)
- **Error handling** with `PdfError` type using `thiserror`
- **Geometry primitives**: `Point`, `Rect`, `IRect`, `Matrix`, `Quad`
- **Buffer** for memory management with MD5 hashing support
- **Stream** abstraction for buffered I/O
- **Colorspace** support (Gray, RGB, CMYK, indexed)
- **Pixmap** for pixel buffer manipulation
- **Document** trait for document abstraction
- **Page** abstraction for page handling

#### PDF Module
- **PDF Object Model**: null, bool, int, real, string, name, array, dict, indirect
- **Compression filters**:
  - FlateDecode (zlib/deflate)
  - LZWDecode
  - ASCII85Decode
  - ASCIIHexDecode
  - RunLengthDecode

#### FFI (C API Compatibility)
- **100% MuPDF API compatible** C headers in `include/mupdf/`
- Handle-based safe resource management
- FFI exports for:
  - Geometry functions (`fz_point`, `fz_rect`, `fz_matrix`, etc.)
  - Context management (`fz_new_context`, `fz_drop_context`)
  - Buffer operations (`fz_new_buffer`, `fz_buffer_len`, etc.)
  - Stream operations (`fz_open_memory`, `fz_read_byte`, etc.)
  - Colorspace functions (`fz_new_colorspace`, `fz_colorspace_n`, etc.)
  - Pixmap functions (`fz_new_pixmap`, `fz_clear_pixmap`, etc.)
  - Document functions (`fz_open_document`, `fz_count_pages`, etc.)
  - PDF object functions (`pdf_new_int`, `pdf_new_dict`, etc.)

#### Optional Features
- `parallel` - Rayon-based parallel processing
- `async` - Tokio-based async I/O
- `jpeg2000` - JPEG 2000 image support

#### Build Targets
- Static library (`libnanopdf.a` / `nanopdf.lib`)
- Dynamic library (`libnanopdf.so` / `nanopdf.dll`)
- Rust library (rlib)

#### Packages
- Debian package support via `cargo-deb`
- RPM package support via `cargo-generate-rpm`

### Notes
- Designed as a drop-in replacement for MuPDF
- Pure Rust implementation with no C dependencies
- MIT/Apache 2.0 dual license (more permissive than MuPDF's AGPL)

[Unreleased]: https://github.com/lexmata/nanopdf/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/lexmata/nanopdf/releases/tag/v0.1.0

