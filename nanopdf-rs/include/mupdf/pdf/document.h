// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's pdf/document.h

#ifndef MUPDF_PDF_DOCUMENT_H
#define MUPDF_PDF_DOCUMENT_H

#include "mupdf/fitz.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Types
// ============================================================================

typedef struct pdf_document pdf_document;
typedef struct pdf_obj pdf_obj;
typedef struct pdf_crypt pdf_crypt;
typedef struct pdf_page pdf_page;
typedef struct pdf_annot pdf_annot;
typedef struct pdf_widget pdf_widget;
typedef struct pdf_xref pdf_xref;
typedef struct pdf_xref_entry pdf_xref_entry;

// ============================================================================
// Document opening
// ============================================================================

pdf_document *pdf_open_document(fz_context *ctx, const char *filename);
pdf_document *pdf_open_document_with_stream(fz_context *ctx, fz_stream *stm);

// ============================================================================
// Reference counting
// ============================================================================

pdf_document *pdf_keep_document(fz_context *ctx, pdf_document *doc);
void pdf_drop_document(fz_context *ctx, pdf_document *doc);

// ============================================================================
// Document properties
// ============================================================================

pdf_document *pdf_document_from_fz_document(fz_context *ctx, fz_document *ptr);
pdf_page *pdf_page_from_fz_page(fz_context *ctx, fz_page *ptr);

int pdf_version(fz_context *ctx, pdf_document *doc);
int pdf_count_pages(fz_context *ctx, pdf_document *doc);
int pdf_count_objects(fz_context *ctx, pdf_document *doc);

// ============================================================================
// Password/encryption
// ============================================================================

int pdf_needs_password(fz_context *ctx, pdf_document *doc);
int pdf_authenticate_password(fz_context *ctx, pdf_document *doc, const char *password);

int pdf_has_permission(fz_context *ctx, pdf_document *doc, int permission);

enum {
    PDF_PERM_PRINT = 1 << 2,
    PDF_PERM_MODIFY = 1 << 3,
    PDF_PERM_COPY = 1 << 4,
    PDF_PERM_ANNOTATE = 1 << 5,
    PDF_PERM_FORM = 1 << 8,
    PDF_PERM_ACCESSIBILITY = 1 << 9,
    PDF_PERM_ASSEMBLE = 1 << 10,
    PDF_PERM_PRINT_HQ = 1 << 11,
};

// ============================================================================
// Object access
// ============================================================================

pdf_obj *pdf_trailer(fz_context *ctx, pdf_document *doc);
pdf_obj *pdf_catalog(fz_context *ctx, pdf_document *doc);

pdf_obj *pdf_load_object(fz_context *ctx, pdf_document *doc, int num);
pdf_obj *pdf_resolve_indirect(fz_context *ctx, pdf_obj *ref);
pdf_obj *pdf_resolve_indirect_chain(fz_context *ctx, pdf_obj *ref);

// ============================================================================
// Page access
// ============================================================================

pdf_page *pdf_load_page(fz_context *ctx, pdf_document *doc, int number);
pdf_page *pdf_keep_page(fz_context *ctx, pdf_page *page);
void pdf_drop_page(fz_context *ctx, pdf_page *page);

pdf_obj *pdf_page_obj(fz_context *ctx, pdf_page *page);
fz_rect pdf_bound_page(fz_context *ctx, pdf_page *page);
fz_matrix pdf_page_transform(fz_context *ctx, pdf_page *page);
int pdf_page_rotation(fz_context *ctx, pdf_page *page);

// ============================================================================
// Stream access
// ============================================================================

fz_buffer *pdf_load_stream(fz_context *ctx, pdf_obj *ref);
fz_buffer *pdf_load_stream_number(fz_context *ctx, pdf_document *doc, int num);
fz_buffer *pdf_load_raw_stream(fz_context *ctx, pdf_obj *ref);
fz_buffer *pdf_load_raw_stream_number(fz_context *ctx, pdf_document *doc, int num);

fz_stream *pdf_open_stream(fz_context *ctx, pdf_obj *ref);
fz_stream *pdf_open_stream_number(fz_context *ctx, pdf_document *doc, int num);
fz_stream *pdf_open_raw_stream(fz_context *ctx, pdf_obj *ref);
fz_stream *pdf_open_raw_stream_number(fz_context *ctx, pdf_document *doc, int num);

// ============================================================================
// Object creation
// ============================================================================

pdf_obj *pdf_add_object(fz_context *ctx, pdf_document *doc, pdf_obj *obj);
pdf_obj *pdf_add_object_drop(fz_context *ctx, pdf_document *doc, pdf_obj *obj);
pdf_obj *pdf_add_stream(fz_context *ctx, pdf_document *doc, fz_buffer *buf, pdf_obj *obj, int compressed);
pdf_obj *pdf_add_new_dict(fz_context *ctx, pdf_document *doc, int initial);
pdf_obj *pdf_add_new_array(fz_context *ctx, pdf_document *doc, int initial);

void pdf_delete_object(fz_context *ctx, pdf_document *doc, int num);

// ============================================================================
// Object update
// ============================================================================

void pdf_update_object(fz_context *ctx, pdf_document *doc, int num, pdf_obj *obj);
void pdf_update_stream(fz_context *ctx, pdf_document *doc, pdf_obj *ref, fz_buffer *buf, int compressed);

// ============================================================================
// Resource lookup
// ============================================================================

pdf_obj *pdf_dict_get_inheritable(fz_context *ctx, pdf_obj *dict, pdf_obj *key);
pdf_obj *pdf_page_resources(fz_context *ctx, pdf_page *page);

// ============================================================================
// Document writing
// ============================================================================

typedef struct pdf_write_options pdf_write_options;

struct pdf_write_options {
    int do_incremental;
    int do_pretty;
    int do_ascii;
    int do_compress;
    int do_compress_images;
    int do_compress_fonts;
    int do_decompress;
    int do_garbage;
    int do_linear;
    int do_clean;
    int do_sanitize;
    int do_appearance;
    int do_encrypt;
    int permissions;
    char opwd_utf8[128];
    char upwd_utf8[128];
};

extern const pdf_write_options pdf_default_write_options;

void pdf_save_document(fz_context *ctx, pdf_document *doc, const char *filename, const pdf_write_options *opts);
void pdf_write_document(fz_context *ctx, pdf_document *doc, fz_output *out, const pdf_write_options *opts);

int pdf_can_be_saved_incrementally(fz_context *ctx, pdf_document *doc);

// ============================================================================
// Journal/Undo
// ============================================================================

int pdf_undoredo_state(fz_context *ctx, pdf_document *doc, int *steps);
int pdf_can_undo(fz_context *ctx, pdf_document *doc);
int pdf_can_redo(fz_context *ctx, pdf_document *doc);
void pdf_undo(fz_context *ctx, pdf_document *doc);
void pdf_redo(fz_context *ctx, pdf_document *doc);

void pdf_begin_operation(fz_context *ctx, pdf_document *doc, const char *descr);
void pdf_begin_implicit_operation(fz_context *ctx, pdf_document *doc);
void pdf_end_operation(fz_context *ctx, pdf_document *doc);
void pdf_abandon_operation(fz_context *ctx, pdf_document *doc);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_PDF_DOCUMENT_H */

