/**
 * MuPDF FFI Compatibility Header
 *
 * This header provides 100% MuPDF-compatible FFI bindings.
 * Include this for drop-in compatibility with MuPDF-based applications.
 *
 * All 660+ functions are available through this header:
 * - fz_*  functions: Core MuPDF-compatible API
 * - pdf_* functions: PDF-specific MuPDF-compatible API  
 * - np_*  functions: Enhanced NanoPDF extensions (watermarks, drawing, optimization)
 *
 * Usage:
 *   #include <mupdf-ffi.h>
 *
 * Or for complete MuPDF compatibility:
 *   #include <mupdf.h>
 */

#ifndef MUPDF_FFI_H
#define MUPDF_FFI_H

#include "nanopdf.h"

/*
 * All MuPDF-compatible functions are available through nanopdf.h
 *
 * MuPDF-Compatible Function Categories (fz_*, pdf_*):
 * - Context management (fz_new_context, fz_drop_context, etc.)
 * - Document operations (fz_open_document, fz_load_page, etc.)
 * - Geometry operations (fz_concat, fz_transform_rect, etc.)
 * - Buffer operations (fz_new_buffer, fz_append_data, etc.)
 * - Device operations (fz_new_bbox_device, fz_fill_path, etc.)
 * - Image operations (fz_new_image_from_pixmap, fz_decode_image, etc.)
 * - Text operations (fz_new_text, fz_show_string, etc.)
 * - PDF object operations (pdf_new_dict, pdf_dict_get, etc.)
 * - PDF annotation operations (pdf_create_annot, pdf_set_annot_contents, etc.)
 * - PDF form operations (pdf_next_widget, pdf_set_field_value, etc.)
 * 
 * Enhanced NanoPDF Functions (np_*):
 * - Document creation (np_add_blank_page, np_write_pdf)
 * - Content addition (np_add_watermark, np_draw_line, np_draw_rectangle, np_draw_circle)
 * - PDF manipulation (np_merge_pdfs, np_split_pdf)
 * - Optimization (np_optimize_pdf, np_linearize_pdf)
 *
 * Total coverage: 660+ functions (650 MuPDF-compatible + 10 enhanced)
 */

#endif /* MUPDF_FFI_H */
