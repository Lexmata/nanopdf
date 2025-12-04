/**
 * NanoPDF - Fast, lightweight PDF library
 *
 * This is a MuPDF-compatible C FFI header for the NanoPDF Rust library.
 * All functions are prefixed with fz_ or pdf_ for compatibility with MuPDF.
 */

#ifndef NANOPDF_H
#define NANOPDF_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

/* Forward declarations - opaque handles */
typedef int32_t fz_context;
typedef int32_t fz_document;
typedef int32_t fz_page;
typedef int32_t fz_device;
typedef int32_t fz_pixmap;
typedef int32_t fz_buffer;
typedef int32_t fz_stream;
typedef int32_t fz_output;
typedef int32_t fz_colorspace;
typedef int32_t fz_font;
typedef int32_t fz_image;
typedef int32_t fz_path;
typedef int32_t fz_text;
typedef int32_t fz_cookie;
typedef int32_t fz_display_list;
typedef int32_t fz_link;
typedef int32_t fz_archive;
typedef int32_t pdf_obj;
typedef int32_t pdf_annot;
typedef int32_t pdf_form_field;

/* Core FFI functions are defined in the compiled library */
/* See the Rust documentation for detailed function signatures */

#ifdef __cplusplus
}
#endif

#endif /* NANOPDF_H */
