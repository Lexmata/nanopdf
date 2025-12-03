// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's context.h

#ifndef MUPDF_FITZ_CONTEXT_H
#define MUPDF_FITZ_CONTEXT_H

#include "mupdf/fitz/version.h"
#include "mupdf/fitz/system.h"
#include "mupdf/fitz/geometry.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Forward declarations
// ============================================================================

typedef struct fz_font_context fz_font_context;
typedef struct fz_colorspace_context fz_colorspace_context;
typedef struct fz_style_context fz_style_context;
typedef struct fz_tuning_context fz_tuning_context;
typedef struct fz_store fz_store;
typedef struct fz_glyph_cache fz_glyph_cache;
typedef struct fz_document_handler_context fz_document_handler_context;
typedef struct fz_archive_handler_context fz_archive_handler_context;
typedef struct fz_output fz_output;
typedef struct fz_context fz_context;

// ============================================================================
// Allocator
// ============================================================================

typedef struct {
    void *user;
    void *(*malloc)(void *, size_t);
    void *(*realloc)(void *, void *, size_t);
    void (*free)(void *, void *);
} fz_alloc_context;

// ============================================================================
// Locks
// ============================================================================

typedef struct {
    void *user;
    void (*lock)(void *user, int lock);
    void (*unlock)(void *user, int lock);
} fz_locks_context;

enum {
    FZ_LOCK_ALLOC = 0,
    FZ_LOCK_FREETYPE,
    FZ_LOCK_GLYPHCACHE,
    FZ_LOCK_MAX
};

// ============================================================================
// Error codes
// ============================================================================

enum {
    FZ_ERROR_NONE = 0,
    FZ_ERROR_MEMORY = 1,
    FZ_ERROR_GENERIC = 2,
    FZ_ERROR_SYNTAX = 3,
    FZ_ERROR_MINOR = 4,
    FZ_ERROR_TRYLATER = 5,
    FZ_ERROR_ABORT = 6,
    FZ_ERROR_SYSTEM = 7,
    FZ_ERROR_LIBRARY = 8,
    FZ_ERROR_FORMAT = 9,
    FZ_ERROR_LIMIT = 10,
    FZ_ERROR_UNSUPPORTED = 11,
    FZ_ERROR_ARGUMENT = 12,
    FZ_ERROR_COUNT
};

// ============================================================================
// Context functions
// ============================================================================

/**
 * Create a new context.
 */
fz_context *fz_new_context(const fz_alloc_context *alloc, const fz_locks_context *locks, size_t max_store);

/**
 * Create a clone of the context (for multi-threading).
 */
fz_context *fz_clone_context(fz_context *ctx);

/**
 * Drop a context reference.
 */
void fz_drop_context(fz_context *ctx);

/**
 * Set user data on context.
 */
void fz_set_user_context(fz_context *ctx, void *user);

/**
 * Get user data from context.
 */
void *fz_user_context(fz_context *ctx);

// ============================================================================
// Exception handling (simplified - Rust uses Result<T,E>)
// ============================================================================

/**
 * Throw an exception.
 */
void fz_throw(fz_context *ctx, int errcode, const char *fmt, ...);
void fz_vthrow(fz_context *ctx, int errcode, const char *fmt, va_list ap);
void fz_rethrow(fz_context *ctx);

/**
 * Get the caught exception message.
 */
const char *fz_caught_message(fz_context *ctx);

/**
 * Get the caught exception code.
 */
int fz_caught(fz_context *ctx);

/**
 * Log a warning.
 */
void fz_warn(fz_context *ctx, const char *fmt, ...);
void fz_vwarn(fz_context *ctx, const char *fmt, va_list ap);

// ============================================================================
// Memory allocation
// ============================================================================

void *fz_malloc(fz_context *ctx, size_t size);
void *fz_malloc_no_throw(fz_context *ctx, size_t size);
void *fz_calloc(fz_context *ctx, size_t count, size_t size);
void *fz_calloc_no_throw(fz_context *ctx, size_t count, size_t size);
void *fz_realloc(fz_context *ctx, void *p, size_t size);
void *fz_realloc_no_throw(fz_context *ctx, void *p, size_t size);
void fz_free(fz_context *ctx, void *p);

char *fz_strdup(fz_context *ctx, const char *s);

#define fz_malloc_struct(CTX, TYPE) \
    ((TYPE*)fz_calloc(CTX, 1, sizeof(TYPE)))

#define fz_malloc_struct_array(CTX, N, TYPE) \
    ((TYPE*)fz_calloc(CTX, N, sizeof(TYPE)))

#define fz_malloc_array(CTX, N, TYPE) \
    ((TYPE*)fz_malloc(CTX, (N) * sizeof(TYPE)))

#define fz_realloc_array(CTX, P, N, TYPE) \
    ((TYPE*)fz_realloc(CTX, P, (N) * sizeof(TYPE)))

// ============================================================================
// Tuning
// ============================================================================

void fz_tune_image_decode(fz_context *ctx, int (*image_decode)(void *, int, int, int, int), void *arg);
void fz_tune_image_scale(fz_context *ctx, fz_irect (*image_scale)(void *, int, int, int, int), void *arg);

// ============================================================================
// Anti-aliasing
// ============================================================================

#define FZ_AA_BITS_MIN 0
#define FZ_AA_BITS_MAX 8

int fz_aa_level(fz_context *ctx);
void fz_set_aa_level(fz_context *ctx, int bits);

int fz_graphics_aa_level(fz_context *ctx);
void fz_set_graphics_aa_level(fz_context *ctx, int bits);

int fz_text_aa_level(fz_context *ctx);
void fz_set_text_aa_level(fz_context *ctx, int bits);

int fz_graphics_min_line_width(fz_context *ctx);
void fz_set_graphics_min_line_width(fz_context *ctx, float min_line_width);

int fz_use_document_css(fz_context *ctx);
void fz_set_use_document_css(fz_context *ctx, int use);

const char *fz_user_css(fz_context *ctx);
void fz_set_user_css(fz_context *ctx, const char *text);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_CONTEXT_H */

