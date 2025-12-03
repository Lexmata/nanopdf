# NanoPDF

A pure Rust PDF library designed as a **drop-in replacement for [MuPDF](https://mupdf.com/)**.

## Overview

NanoPDF is a complete reimplementation of the MuPDF library in safe Rust. It provides:

- **100% API compatibility** with MuPDF's C headers
- **No unsafe Rust** in the core implementation
- **Pure Rust** - no C dependencies or FFI bindings to MuPDF
- **MIT/Apache 2.0 dual license** - more permissive than MuPDF's AGPL

### Why NanoPDF?

MuPDF is an excellent PDF library, but its AGPL license can be problematic for commercial applications. NanoPDF provides the same API with a permissive license, allowing you to:

- Use it in proprietary applications without open-sourcing your code
- Avoid MuPDF's commercial licensing fees
- Benefit from Rust's memory safety guarantees
- Deploy to WebAssembly and other Rust-supported targets

### Drop-in Replacement

NanoPDF's C headers (`include/mupdf/*.h`) mirror MuPDF's API exactly. Existing C/C++ code using MuPDF can switch to NanoPDF by:

1. Replacing MuPDF headers with NanoPDF headers
2. Linking against `libnanopdf.a` instead of `libmupdf.a`

No code changes required.

## Features

- PDF parsing and object model
- Geometry primitives (Point, Rect, Matrix, Quad)
- Buffer and stream abstractions
- Colorspace and pixmap support
- Document and page handling
- Annotations and form fields
- Optional parallel processing with `rayon`
- Optional async I/O with `tokio`

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
nanopdf = "0.1"
```

### Optional Features

```toml
[dependencies]
nanopdf = { version = "0.1", features = ["parallel", "async"] }
```

- `parallel` - Enable parallel processing using rayon
- `async` - Enable async I/O using tokio
- `jpeg2000` - Enable JPEG 2000 support

## Usage

```rust
use nanopdf::fitz::buffer::Buffer;
use nanopdf::fitz::stream::Stream;
use nanopdf::fitz::geometry::{Point, Rect, Matrix};

// Create a buffer
let buffer = Buffer::from_slice(b"Hello, PDF!");

// Open a stream from memory
let mut stream = Stream::open_memory(b"PDF data here");

// Work with geometry
let point = Point::new(100.0, 200.0);
let rect = Rect::new(0.0, 0.0, 612.0, 792.0); // US Letter
let matrix = Matrix::scale(2.0, 2.0);
```

## Building Static Libraries

The library can be built as a static library for C/C++ integration:

```bash
cargo build --release
```

This produces:
- `target/release/libnanopdf.a` (Unix)
- `target/release/nanopdf.lib` (Windows MSVC)

## License

Dual-licensed under MIT or Apache 2.0.

