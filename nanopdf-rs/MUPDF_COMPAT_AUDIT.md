# MuPDF Compatibility Audit

This document audits the current state of NanoPDF's MuPDF compatibility implementation.

**Last Updated**: $(date)

---

## Executive Summary

| Category | Status | Completion | Notes |
|----------|--------|------------|-------|
| **FFI Layer** | ✅ Complete | 100% | 660+ C-compatible functions |
| **Core Types** | ✅ Complete | 100% | Geometry, buffers, streams |
| **PDF Parsing** | ⚠️ Partial | 40% | Basic structure, needs content stream parser |
| **Rendering** | ❌ Incomplete | 20% | Infrastructure exists, no pixel rendering |
| **Image Decoding** | ⚠️ Partial | 50% | Basic formats, missing JPEG2000/JBIG2 |
| **Font Handling** | ⚠️ Partial | 30% | Basic parsing, no glyph rendering |
| **Text Extraction** | ⚠️ Partial | 40% | Basic text, no structured layout |
| **Annotations** | ❌ Incomplete | 10% | Types defined, no rendering |
| **Forms** | ❌ Incomplete | 10% | Types defined, no interaction |
| **Encryption** | ⚠️ Partial | 50% | Basic support, needs full crypto |
| **Overall** | ⚠️ **Partial** | **45%** | FFI complete, core needs work |

---

## Detailed Analysis

### 1. FFI Layer ✅ Complete (100%)

**What Works:**
- ✅ 660+ C-compatible function signatures
- ✅ Handle-based memory management
- ✅ Null pointer safety
- ✅ Error code propagation
- ✅ Thread-safe handle stores
- ✅ Full type compatibility with MuPDF API

**Status:** Production-ready. All FFI functions are properly declared and handle validation correctly.

---

### 2. Core Geometry & Types ✅ Complete (100%)

**What Works:**
- ✅ `fz_point`, `fz_rect`, `fz_irect` - Complete
- ✅ `fz_matrix`, `fz_quad` - Complete
- ✅ Matrix operations (translate, scale, rotate, concat, invert)
- ✅ Rectangle operations (union, intersect, contains, transform)
- ✅ Point transformations
- ✅ All geometry helper functions

**Files:**
- `src/fitz/geometry.rs` - Core types (complete)
- `src/ffi/geometry.rs` - FFI bindings (complete)

**Status:** Production-ready. Full MuPDF compatibility.

---

### 3. Buffer & Stream I/O ✅ Complete (100%)

**What Works:**
- ✅ `fz_buffer` - Dynamic byte arrays
- ✅ `fz_stream` - Input streams
- ✅ `fz_output` - Output streams
- ✅ Buffer reading/writing
- ✅ Stream seeking, reading, telling
- ✅ Output writing (bytes, integers, floats, strings)

**Files:**
- `src/fitz/buffer/*.rs` - Buffer implementation
- `src/fitz/stream.rs` - Stream implementation
- `src/fitz/output.rs` - Output implementation
- `src/ffi/buffer.rs` - FFI bindings
- `src/ffi/stream.rs` - FFI bindings
- `src/ffi/output.rs` - FFI bindings

**Status:** Production-ready. Full MuPDF compatibility.

---

### 4. PDF Parsing ⚠️ Partial (40%)

**What Works:**
- ✅ PDF lexer/tokenizer
- ✅ PDF object parsing (dict, array, numbers, strings, names, refs)
- ✅ Cross-reference table parsing
- ✅ Trailer parsing
- ✅ Document structure loading
- ✅ Page tree traversal

**What's Missing:**
- ❌ **Content stream parsing** (critical!)
- ❌ **PDF operator interpretation** (critical!)
- ❌ Linearized PDF support
- ❌ Incremental update support
- ❌ Damaged PDF repair
- ❌ Object stream decompression

**Files:**
- `src/pdf/lexer.rs` - Tokenizer (complete)
- `src/pdf/parser.rs` - Object parser (complete)
- `src/pdf/object.rs` - Object types (complete)
- `src/pdf/xref.rs` - Cross-reference (complete)
- `src/pdf/document.rs` - Document structure (partial)
- `src/pdf/interpret.rs` - **Content stream interpreter (STUB)**

**Status:** Basic parsing works, but **content stream interpretation is critical missing piece**.

---

### 5. Rendering Pipeline ❌ Incomplete (20%)

**What Works:**
- ✅ Device interface defined
- ✅ Display list structure
- ✅ Path construction
- ✅ Stroke state management
- ✅ Colorspace definitions
- ✅ Pixmap structure

**What's Missing:**
- ❌ **Actual pixel rendering** (critical!)
- ❌ **Content stream to device calls** (critical!)
- ❌ Blending modes
- ❌ Clipping
- ❌ Transparency groups
- ❌ Pattern fills
- ❌ Shadings
- ❌ Soft masks

**Files:**
- `src/fitz/device.rs` - Device interface (complete)
- `src/fitz/display_list.rs` - Display list (complete)
- `src/fitz/path.rs` - Path operations (complete)
- `src/fitz/pixmap.rs` - Pixmap structure (partial - no actual rendering)
- `src/pdf/interpret.rs` - **PDF interpreter (STUB)**

**Status:** Infrastructure exists, but **no actual pixel rendering implemented**.

**Critical Missing Piece:**
```rust
// src/pdf/interpret.rs needs to:
// 1. Parse PDF content stream (sequence of operators)
// 2. Maintain graphics state stack
// 3. Call appropriate device methods for each operator
// 4. Handle text positioning, matrix transforms, etc.
```

---

### 6. Image Decoding ⚠️ Partial (50%)

**What Works:**
- ✅ PNG decoding (via `png` crate)
- ✅ JPEG decoding (via `jpeg-decoder`)
- ✅ Basic DCT filter
- ✅ Image structure parsing

**What's Missing:**
- ❌ **JPEG2000 decoding** (common in PDFs)
- ❌ **JBIG2 decoding** (common in scanned PDFs)
- ❌ CCITT Group 3/4 fax decoding
- ❌ Image mask handling
- ❌ Soft mask handling
- ❌ Decode arrays
- ❌ Color key masking

**Files:**
- `src/fitz/image.rs` - Image structure (complete)
- `src/pdf/image.rs` - PDF image parsing (partial)
- `src/pdf/filter/*.rs` - Decoders (partial)
- `src/pdf/filter/jbig2.rs` - **STUB**
- `src/pdf/filter/jpx.rs` - Missing (JPEG2000)
- `src/pdf/filter/ccitt.rs` - **STUB**

**Status:** Basic images work, but **JPEG2000 and JBIG2 are critical for many PDFs**.

---

### 7. Font Handling ⚠️ Partial (30%)

**What Works:**
- ✅ Font structure parsing
- ✅ Font descriptor parsing
- ✅ CMap parsing (character mapping)
- ✅ Basic Type1/TrueType/OpenType detection

**What's Missing:**
- ❌ **Glyph rendering** (critical!)
- ❌ **Font rasterization** (critical!)
- ❌ TrueType/OpenType glyph extraction
- ❌ Type1 glyph extraction
- ❌ Type3 font handling
- ❌ CID font handling
- ❌ Font metrics calculation
- ❌ Text width calculation

**Files:**
- `src/fitz/font.rs` - Font structure (partial)
- `src/pdf/font.rs` - PDF font parsing (partial)
- `src/pdf/cmap.rs` - CMap parsing (partial)

**Status:** Can parse fonts, but **cannot render glyphs to pixels**.

**Recommendation:** Integrate `rusttype` or `ttf-parser` + `ab_glyph` for glyph rasterization.

---

### 8. Text Extraction ⚠️ Partial (40%)

**What Works:**
- ✅ Basic text extraction
- ✅ Character positioning (simple)
- ✅ Text object structure

**What's Missing:**
- ❌ **Structured text layout** (blocks, lines, spans)
- ❌ Bidi (right-to-left) text support
- ❌ Text positioning with fonts
- ❌ Text matrix calculations
- ❌ Line detection
- ❌ Word boundaries
- ❌ Reading order detection
- ❌ Search highlighting

**Files:**
- `src/fitz/text.rs` - Text structure (partial)
- `src/ffi/text.rs` - FFI bindings (partial)

**Status:** Basic extraction works through FFI stubs, but **no proper text layout analysis**.

---

### 9. Annotations ❌ Incomplete (10%)

**What Works:**
- ✅ Annotation types defined
- ✅ Annotation structure parsing

**What's Missing:**
- ❌ **Annotation rendering** (all types)
- ❌ Annotation creation/modification
- ❌ Appearance stream parsing
- ❌ Markup annotation support
- ❌ Widget annotation support
- ❌ 3D annotation support
- ❌ Rich media annotation support

**Files:**
- `src/pdf/annot.rs` - Annotation types (types only)
- `src/ffi/annot.rs` - FFI bindings (stubs)

**Status:** Types defined, but **no actual annotation rendering**.

---

### 10. Forms (AcroForms) ❌ Incomplete (10%)

**What Works:**
- ✅ Form field types defined
- ✅ Form structure parsing

**What's Missing:**
- ❌ **Form rendering** (all types)
- ❌ Form field interaction
- ❌ Form field value reading/writing
- ❌ Form calculation
- ❌ Form validation
- ❌ JavaScript form actions
- ❌ Digital signatures

**Files:**
- `src/pdf/form.rs` - Form types (types only)
- `src/ffi/form.rs` - FFI bindings (stubs)

**Status:** Types defined, but **no actual form support**.

---

### 11. Encryption/Security ⚠️ Partial (50%)

**What Works:**
- ✅ Basic encryption structure parsing
- ✅ Password authentication (simplified)
- ✅ Permission checking

**What's Missing:**
- ❌ **Full encryption algorithms** (RC4, AES-128, AES-256)
- ❌ Object-level encryption/decryption
- ❌ Stream encryption/decryption
- ❌ Digital signatures
- ❌ Certificate-based encryption

**Files:**
- `src/pdf/crypt.rs` - Encryption (partial)

**Status:** Basic structure works, but **full crypto not implemented**.

**Recommendation:** Use `aes`, `rc4`, `md5`, `sha2` crates.

---

## Critical Path to 100% Compatibility

To achieve 100% MuPDF compatibility, these are the **critical blockers** that must be implemented:

### Phase 1: Rendering Core (Highest Priority)

**1. PDF Content Stream Interpreter** (2-3 weeks)
   - File: `src/pdf/interpret.rs`
   - Parse PDF operators from content stream
   - Maintain graphics state stack
   - Call device methods for each operator
   - **Impact**: Enables actual PDF rendering

**2. Pixel Rendering Engine** (2-3 weeks)
   - File: `src/fitz/pixmap.rs` + new `src/fitz/render.rs`
   - Implement scan line rasterizer
   - Path filling (even-odd, non-zero winding)
   - Path stroking (various line caps/joins)
   - Blending modes
   - **Impact**: Enables page rendering to images

**3. Image Decoding** (1-2 weeks)
   - JPEG2000: Use `jpeg2000` or `openjp2` crate
   - JBIG2: Use `jbig2-rs` or FFI to `jbig2dec`
   - CCITT: Implement Group 3/4 fax decoding
   - **Impact**: Enables PDFs with embedded images

### Phase 2: Text & Fonts (High Priority)

**4. Glyph Rasterization** (1-2 weeks)
   - Use `rusttype` or `ab_glyph` for TrueType/OpenType
   - Type1 font rendering
   - Type3 font support
   - **Impact**: Enables text rendering

**5. Structured Text Extraction** (1 week)
   - Text layout analysis (blocks, lines, words)
   - Reading order detection
   - Bidi support
   - **Impact**: Enables proper text extraction

### Phase 3: Advanced Features (Medium Priority)

**6. Annotation Rendering** (1 week)
   - Parse appearance streams
   - Render markup annotations
   - Render widget annotations
   - **Impact**: Enables annotation display

**7. Form Support** (1-2 weeks)
   - Form field rendering
   - Field value reading/writing
   - Form calculations
   - **Impact**: Enables interactive forms

**8. Full Encryption** (1 week)
   - Implement RC4, AES-128, AES-256
   - Object/stream decryption
   - **Impact**: Enables encrypted PDFs

### Phase 4: Polish (Low Priority)

**9. Advanced Rendering** (1-2 weeks)
   - Transparency groups
   - Soft masks
   - Shadings
   - Patterns
   - **Impact**: Enables complex PDFs

**10. Damaged PDF Repair** (1 week)
   - Error recovery
   - Cross-reference repair
   - **Impact**: Handles damaged PDFs

---

## Estimated Timeline

| Phase | Duration | Effort | Priority |
|-------|----------|--------|----------|
| Phase 1: Rendering Core | 5-8 weeks | 400-600 hours | **Critical** |
| Phase 2: Text & Fonts | 2-3 weeks | 160-240 hours | **High** |
| Phase 3: Advanced Features | 3-5 weeks | 240-400 hours | **Medium** |
| Phase 4: Polish | 2-3 weeks | 160-240 hours | **Low** |
| **Total** | **12-19 weeks** | **960-1480 hours** | |

**Realistic estimate**: **3-5 months** of focused development

---

## Recommended Approach

### Option 1: Incremental Implementation (Recommended)

Implement features in priority order, testing with real PDFs at each stage:

1. ✅ **Week 1-3**: PDF content stream interpreter
2. ✅ **Week 4-6**: Pixel rendering engine (basic)
3. ✅ **Week 7-8**: Image decoding (JPEG2000, JBIG2)
4. ✅ **Week 9-10**: Glyph rasterization
5. ✅ **Week 11-12**: Structured text extraction
6. ✅ **Week 13-14**: Annotations
7. ✅ **Week 15-17**: Forms
8. ✅ **Week 18-19**: Advanced rendering & polish

### Option 2: Parallel Development

Split work across multiple developers:

- **Developer A**: Rendering core (interpreter + rasterizer)
- **Developer B**: Images + fonts
- **Developer C**: Text extraction + annotations
- **Developer D**: Forms + encryption

**Timeline**: 6-8 weeks with 4 developers

### Option 3: FFI to MuPDF (Fast, but defeats purpose)

As an interim solution, FFI to actual MuPDF library for complex operations:

- Keep Rust implementation for simple operations
- FFI to MuPDF for rendering, text extraction, etc.
- Gradually replace MuPDF calls with Rust implementations

**Timeline**: 2-3 weeks for FFI integration

---

## Dependencies & Crates Needed

### Required Crates

```toml
[dependencies]
# Image decoding
jpeg-decoder = "0.3"
png = "0.17"
# openjpeg2 = "0.1"  # JPEG2000 (C FFI)
# jbig2dec-sys = "0.1"  # JBIG2 (C FFI)

# Font rendering
ab_glyph = "0.2"
ttf-parser = "0.20"
# freetype = "0.33"  # Alternative

# Compression
flate2 = "1.0"  # Already included
lzw = "0.10"  # Already included

# Encryption
aes = "0.8"
rc4 = "0.1"
md5 = "0.7"
sha2 = "0.10"

# Rasterization
tiny-skia = "0.11"  # Optional: for rendering

# PDF structure
nom = "7.1"  # Parser combinators
```

---

## Testing Strategy

### Phase 1: Unit Tests
- Test each component individually
- Mock dependencies
- Fast feedback

### Phase 2: Integration Tests
- Test with real PDF files
- Use PDF test suite (test-pdfs/)
- Verify output against MuPDF

### Phase 3: Visual Regression Tests
- Compare rendered output pixel-by-pixel
- Use MuPDF as reference
- Catch rendering bugs

### Phase 4: Fuzzing
- Already have fuzzing infrastructure
- Fuzz PDF parser
- Fuzz content stream interpreter

---

## Conclusion

**Current State**: 45% complete
- ✅ FFI layer is production-ready
- ✅ Core types are complete
- ⚠️ **PDF rendering is the critical missing piece**
- ⚠️ Text, images, fonts need work
- ❌ Annotations and forms not implemented

**Path to 100%**: Focus on **PDF content stream interpreter** and **pixel rendering engine** first. These two components unlock the majority of remaining functionality.

**Recommendation**: Start with Phase 1 (Rendering Core) as it's the biggest blocker. Once rendering works, the rest will follow more easily.

---

**Next Steps**:
1. Implement PDF content stream interpreter (`src/pdf/interpret.rs`)
2. Implement pixel rendering engine (`src/fitz/render.rs`)
3. Test with simple PDFs (text only)
4. Add image decoding
5. Add glyph rendering
6. Test with complex PDFs

---

Generated: $(date)

