// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's buffer.h

#ifndef MUPDF_FITZ_BUFFER_H
#define MUPDF_FITZ_BUFFER_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Buffer type
// ============================================================================

typedef struct fz_buffer fz_buffer;

// ============================================================================
// Buffer functions
// ============================================================================

/**
 * Take an additional reference to the buffer.
 */
fz_buffer *fz_keep_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Drop a reference to the buffer.
 */
void fz_drop_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Retrieve internal memory of buffer.
 * Returns the current size of the data in bytes.
 */
size_t fz_buffer_storage(fz_context *ctx, fz_buffer *buf, unsigned char **datap);

/**
 * Ensure that a buffer's data ends in a 0 byte, and return a pointer to it.
 */
const char *fz_string_from_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Create a new buffer with given capacity.
 */
fz_buffer *fz_new_buffer(fz_context *ctx, size_t capacity);

/**
 * Create a new buffer with existing data (takes ownership).
 */
fz_buffer *fz_new_buffer_from_data(fz_context *ctx, unsigned char *data, size_t size);

/**
 * Create a new buffer with shared data (does not take ownership).
 */
fz_buffer *fz_new_buffer_from_shared_data(fz_context *ctx, const unsigned char *data, size_t size);

/**
 * Create a new buffer containing a copy of the passed data.
 */
fz_buffer *fz_new_buffer_from_copied_data(fz_context *ctx, const unsigned char *data, size_t size);

/**
 * Make a new buffer, containing a copy of the data.
 */
fz_buffer *fz_clone_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Create a new buffer with data decoded from base64.
 */
fz_buffer *fz_new_buffer_from_base64(fz_context *ctx, const char *data, size_t size);

/**
 * Ensure buffer has given capacity.
 */
void fz_resize_buffer(fz_context *ctx, fz_buffer *buf, size_t capacity);

/**
 * Make some space within a buffer.
 */
void fz_grow_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Trim wasted capacity from a buffer.
 */
void fz_trim_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Empties the buffer.
 */
void fz_clear_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Create a new buffer with a subset of the data.
 */
fz_buffer *fz_slice_buffer(fz_context *ctx, fz_buffer *buf, int64_t start, int64_t end);

/**
 * Append the contents of source buffer onto destination.
 */
void fz_append_buffer(fz_context *ctx, fz_buffer *destination, fz_buffer *source);

/**
 * Write base64 encoded data block.
 */
void fz_append_base64(fz_context *ctx, fz_buffer *out, const unsigned char *data, size_t size, int newline);

/**
 * Append a base64 encoded fz_buffer.
 */
void fz_append_base64_buffer(fz_context *ctx, fz_buffer *out, fz_buffer *data, int newline);

/**
 * Append data to a buffer.
 */
void fz_append_data(fz_context *ctx, fz_buffer *buf, const void *data, size_t len);
void fz_append_string(fz_context *ctx, fz_buffer *buf, const char *data);
void fz_append_byte(fz_context *ctx, fz_buffer *buf, int c);
void fz_append_rune(fz_context *ctx, fz_buffer *buf, int c);
void fz_append_int32_le(fz_context *ctx, fz_buffer *buf, int x);
void fz_append_int16_le(fz_context *ctx, fz_buffer *buf, int x);
void fz_append_int32_be(fz_context *ctx, fz_buffer *buf, int x);
void fz_append_int16_be(fz_context *ctx, fz_buffer *buf, int x);
void fz_append_bits(fz_context *ctx, fz_buffer *buf, int value, int count);
void fz_append_bits_pad(fz_context *ctx, fz_buffer *buf);

/**
 * Append a string with PDF syntax quotes and escapes.
 */
void fz_append_pdf_string(fz_context *ctx, fz_buffer *buffer, const char *text);

/**
 * Format and append data to buffer using printf-like formatting.
 */
void fz_append_printf(fz_context *ctx, fz_buffer *buffer, const char *fmt, ...);
void fz_append_vprintf(fz_context *ctx, fz_buffer *buffer, const char *fmt, va_list args);

/**
 * Zero-terminate buffer.
 */
void fz_terminate_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Create an MD5 digest from buffer contents.
 */
void fz_md5_buffer(fz_context *ctx, fz_buffer *buffer, unsigned char digest[16]);

/**
 * Take ownership of buffer contents.
 */
size_t fz_buffer_extract(fz_context *ctx, fz_buffer *buf, unsigned char **data);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_BUFFER_H */

