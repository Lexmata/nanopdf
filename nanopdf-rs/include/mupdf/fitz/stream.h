// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's stream.h

#ifndef MUPDF_FITZ_STREAM_H
#define MUPDF_FITZ_STREAM_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"
#include "mupdf/fitz/buffer.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Stream type
// ============================================================================

typedef struct fz_stream fz_stream;

// ============================================================================
// Stream functions
// ============================================================================

/**
 * Take an additional reference to the stream.
 */
fz_stream *fz_keep_stream(fz_context *ctx, fz_stream *stm);

/**
 * Drop a reference to the stream.
 */
void fz_drop_stream(fz_context *ctx, fz_stream *stm);

/**
 * Open a file for reading.
 */
fz_stream *fz_open_file(fz_context *ctx, const char *filename);

/**
 * Open a stream from memory.
 */
fz_stream *fz_open_memory(fz_context *ctx, const unsigned char *data, size_t len);

/**
 * Open a stream from a buffer.
 */
fz_stream *fz_open_buffer(fz_context *ctx, fz_buffer *buf);

/**
 * Read from a stream into a buffer.
 */
size_t fz_read(fz_context *ctx, fz_stream *stm, unsigned char *data, size_t len);

/**
 * Read a single byte from a stream.
 */
int fz_read_byte(fz_context *ctx, fz_stream *stm);

/**
 * Peek at the next byte in the stream.
 */
int fz_peek_byte(fz_context *ctx, fz_stream *stm);

/**
 * Unread a byte.
 */
void fz_unread_byte(fz_context *ctx, fz_stream *stm);

/**
 * Check if the stream is at EOF.
 */
int fz_is_eof(fz_context *ctx, fz_stream *stm);

/**
 * Skip whitespace and comments.
 */
int fz_skip_space(fz_context *ctx, fz_stream *stm);

/**
 * Skip to the end of line.
 */
int fz_skip_string(fz_context *ctx, fz_stream *stm, const char *str);

/**
 * Read all available data into a buffer.
 */
fz_buffer *fz_read_all(fz_context *ctx, fz_stream *stm, size_t initial);

/**
 * Read a line from the stream.
 */
char *fz_read_line(fz_context *ctx, fz_stream *stm, char *buf, size_t max);

/**
 * Seek to a position in the stream.
 */
void fz_seek(fz_context *ctx, fz_stream *stm, int64_t offset, int whence);

/**
 * Get the current position in the stream.
 */
int64_t fz_tell(fz_context *ctx, fz_stream *stm);

/**
 * Read integers with specific endianness.
 */
uint16_t fz_read_uint16(fz_context *ctx, fz_stream *stm);
uint32_t fz_read_uint24(fz_context *ctx, fz_stream *stm);
uint32_t fz_read_uint32(fz_context *ctx, fz_stream *stm);
uint64_t fz_read_uint64(fz_context *ctx, fz_stream *stm);

uint16_t fz_read_uint16_le(fz_context *ctx, fz_stream *stm);
uint32_t fz_read_uint24_le(fz_context *ctx, fz_stream *stm);
uint32_t fz_read_uint32_le(fz_context *ctx, fz_stream *stm);
uint64_t fz_read_uint64_le(fz_context *ctx, fz_stream *stm);

int16_t fz_read_int16(fz_context *ctx, fz_stream *stm);
int32_t fz_read_int32(fz_context *ctx, fz_stream *stm);
int64_t fz_read_int64(fz_context *ctx, fz_stream *stm);

int16_t fz_read_int16_le(fz_context *ctx, fz_stream *stm);
int32_t fz_read_int32_le(fz_context *ctx, fz_stream *stm);
int64_t fz_read_int64_le(fz_context *ctx, fz_stream *stm);

float fz_read_float(fz_context *ctx, fz_stream *stm);
float fz_read_float_le(fz_context *ctx, fz_stream *stm);

/**
 * Read bits from a stream.
 */
unsigned int fz_read_bits(fz_context *ctx, fz_stream *stm, int n);
void fz_sync_bits(fz_context *ctx, fz_stream *stm);
int fz_is_eof_bits(fz_context *ctx, fz_stream *stm);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_STREAM_H */

