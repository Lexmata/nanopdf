// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's document.h

#ifndef MUPDF_FITZ_DOCUMENT_H
#define MUPDF_FITZ_DOCUMENT_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"
#include "mupdf/fitz/geometry.h"
#include "mupdf/fitz/device.h"
#include "mupdf/fitz/buffer.h"
#include "mupdf/fitz/stream.h"
#include "mupdf/fitz/link.h"
#include "mupdf/fitz/outline.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Types
// ============================================================================

typedef struct fz_document fz_document;
typedef struct fz_page fz_page;
typedef struct fz_link fz_link;
typedef struct fz_outline fz_outline;
typedef struct fz_location fz_location;
typedef struct fz_bookmark fz_bookmark;

struct fz_location {
    int chapter;
    int page;
};

typedef intptr_t fz_bookmark;

// ============================================================================
// Document opening
// ============================================================================

/**
 * Open a document from a filename.
 */
fz_document *fz_open_document(fz_context *ctx, const char *filename);

/**
 * Open a document from a stream.
 */
fz_document *fz_open_document_with_stream(fz_context *ctx, const char *magic, fz_stream *stream);

/**
 * Open a document from a buffer.
 */
fz_document *fz_open_document_with_buffer(fz_context *ctx, const char *magic, fz_buffer *buffer);

// ============================================================================
// Reference counting
// ============================================================================

fz_document *fz_keep_document(fz_context *ctx, fz_document *doc);
void fz_drop_document(fz_context *ctx, fz_document *doc);

// ============================================================================
// Document properties
// ============================================================================

int fz_needs_password(fz_context *ctx, fz_document *doc);
int fz_authenticate_password(fz_context *ctx, fz_document *doc, const char *password);

int fz_has_permission(fz_context *ctx, fz_document *doc, int permission);

enum {
    FZ_PERMISSION_PRINT = (1 << 0),
    FZ_PERMISSION_COPY = (1 << 1),
    FZ_PERMISSION_EDIT = (1 << 2),
    FZ_PERMISSION_ANNOTATE = (1 << 3),
};

// ============================================================================
// Page counting
// ============================================================================

int fz_count_pages(fz_context *ctx, fz_document *doc);
int fz_count_chapters(fz_context *ctx, fz_document *doc);
int fz_count_chapter_pages(fz_context *ctx, fz_document *doc, int chapter);

fz_location fz_last_page(fz_context *ctx, fz_document *doc);
fz_location fz_next_page(fz_context *ctx, fz_document *doc, fz_location loc);
fz_location fz_previous_page(fz_context *ctx, fz_document *doc, fz_location loc);
fz_location fz_clamp_location(fz_context *ctx, fz_document *doc, fz_location loc);

int fz_page_number_from_location(fz_context *ctx, fz_document *doc, fz_location loc);
fz_location fz_location_from_page_number(fz_context *ctx, fz_document *doc, int number);

// ============================================================================
// Page loading
// ============================================================================

fz_page *fz_load_page(fz_context *ctx, fz_document *doc, int number);
fz_page *fz_load_chapter_page(fz_context *ctx, fz_document *doc, int chapter, int page);

fz_page *fz_keep_page(fz_context *ctx, fz_page *page);
void fz_drop_page(fz_context *ctx, fz_page *page);

// ============================================================================
// Page properties
// ============================================================================

fz_rect fz_bound_page(fz_context *ctx, fz_page *page);
void fz_run_page(fz_context *ctx, fz_page *page, fz_device *dev, fz_matrix transform, fz_cookie *cookie);
void fz_run_page_contents(fz_context *ctx, fz_page *page, fz_device *dev, fz_matrix transform, fz_cookie *cookie);
void fz_run_page_annots(fz_context *ctx, fz_page *page, fz_device *dev, fz_matrix transform, fz_cookie *cookie);
void fz_run_page_widgets(fz_context *ctx, fz_page *page, fz_device *dev, fz_matrix transform, fz_cookie *cookie);

fz_link *fz_load_links(fz_context *ctx, fz_page *page);
char *fz_page_label(fz_context *ctx, fz_page *page, char *buf, size_t size);

// ============================================================================
// Outline/bookmarks
// ============================================================================

fz_outline *fz_load_outline(fz_context *ctx, fz_document *doc);
void fz_drop_outline(fz_context *ctx, fz_outline *outline);

fz_bookmark fz_make_bookmark(fz_context *ctx, fz_document *doc, fz_location loc);
fz_location fz_lookup_bookmark(fz_context *ctx, fz_document *doc, fz_bookmark mark);

// ============================================================================
// Metadata
// ============================================================================

int fz_lookup_metadata(fz_context *ctx, fz_document *doc, const char *key, char *buf, int size);
void fz_set_metadata(fz_context *ctx, fz_document *doc, const char *key, const char *value);

#define FZ_META_FORMAT "format"
#define FZ_META_ENCRYPTION "encryption"
#define FZ_META_INFO_TITLE "info:Title"
#define FZ_META_INFO_AUTHOR "info:Author"
#define FZ_META_INFO_SUBJECT "info:Subject"
#define FZ_META_INFO_KEYWORDS "info:Keywords"
#define FZ_META_INFO_CREATOR "info:Creator"
#define FZ_META_INFO_PRODUCER "info:Producer"
#define FZ_META_INFO_CREATIONDATE "info:CreationDate"
#define FZ_META_INFO_MODIFICATIONDATE "info:ModDate"

// ============================================================================
// Cookie for progress/abort
// ============================================================================

typedef struct fz_cookie fz_cookie;

struct fz_cookie {
    int abort;
    int progress;
    size_t progress_max;
    int errors;
    int incomplete;
};

// ============================================================================
// Output intent
// ============================================================================

fz_colorspace *fz_document_output_intent(fz_context *ctx, fz_document *doc);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_DOCUMENT_H */

