// NanoPDF - MuPDF API Compatible C Header
// Auto-generated from Rust FFI - DO NOT EDIT MANUALLY
// Module: link

#ifndef MUPDF_FITZ_LINK_H
#define MUPDF_FITZ_LINK_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Link Functions (8 total)
// ============================================================================

void fz_drop_link(int32_t _ctx, int32_t link);
int fz_link_is_external(int32_t _ctx, int32_t link);
int fz_link_is_valid(int32_t _ctx, int32_t link);
fz_rect fz_link_rect(int32_t _ctx, int32_t link);
void fz_link_uri(int32_t _ctx, int32_t link, char * buffer, size_t size);
int32_t fz_load_links(int32_t _ctx, int32_t _page);
int32_t fz_next_link(int32_t _ctx, int32_t _link);
int fz_resolve_link_page(int32_t _ctx, int32_t _doc, int32_t link);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_LINK_H */
