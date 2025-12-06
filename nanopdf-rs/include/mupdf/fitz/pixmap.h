// NanoPDF - MuPDF API Compatible C Header
// Auto-generated from Rust FFI - DO NOT EDIT MANUALLY
// Module: pixmap

#ifndef MUPDF_FITZ_PIXMAP_H
#define MUPDF_FITZ_PIXMAP_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Pixmap Functions (8 total)
// ============================================================================

void fz_clear_pixmap(int32_t _ctx, int32_t pixmap, int value);
void fz_drop_pixmap(int32_t _ctx, int32_t pixmap);
int32_t fz_new_pixmap(int32_t _ctx, int32_t colorspace, int w, int h, int alpha);
int fz_pixmap_components(int32_t _ctx, int32_t pixmap);
int fz_pixmap_height(int32_t _ctx, int32_t pixmap);
void fz_pixmap_samples(int32_t _ctx, int32_t pixmap, *const c_uchar * data, size_t * size);
size_t fz_pixmap_stride(int32_t _ctx, int32_t pixmap);
int fz_pixmap_width(int32_t _ctx, int32_t pixmap);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_PIXMAP_H */
