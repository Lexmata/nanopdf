// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's pixmap.h

#ifndef MUPDF_FITZ_PIXMAP_H
#define MUPDF_FITZ_PIXMAP_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"
#include "mupdf/fitz/geometry.h"
#include "mupdf/fitz/color.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Types
// ============================================================================

typedef struct fz_pixmap fz_pixmap;
typedef struct fz_colorspace fz_colorspace;
typedef struct fz_separations fz_separations;

// ============================================================================
// Pixmap creation
// ============================================================================

fz_pixmap *fz_new_pixmap(fz_context *ctx, fz_colorspace *cs, int w, int h, fz_separations *seps, int alpha);
fz_pixmap *fz_new_pixmap_with_bbox(fz_context *ctx, fz_colorspace *cs, fz_irect bbox, fz_separations *seps, int alpha);
fz_pixmap *fz_new_pixmap_with_data(fz_context *ctx, fz_colorspace *cs, int w, int h, fz_separations *seps, int alpha, int stride, unsigned char *samples);
fz_pixmap *fz_new_pixmap_with_bbox_and_data(fz_context *ctx, fz_colorspace *cs, fz_irect bbox, fz_separations *seps, int alpha, unsigned char *samples);
fz_pixmap *fz_new_pixmap_from_pixmap(fz_context *ctx, fz_pixmap *pixmap, fz_irect *rect);

// ============================================================================
// Reference counting
// ============================================================================

fz_pixmap *fz_keep_pixmap(fz_context *ctx, fz_pixmap *pix);
void fz_drop_pixmap(fz_context *ctx, fz_pixmap *pix);

// ============================================================================
// Pixmap properties
// ============================================================================

fz_colorspace *fz_pixmap_colorspace(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_x(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_y(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_width(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_height(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_components(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_colorants(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_spots(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_alpha(fz_context *ctx, fz_pixmap *pix);
int fz_pixmap_stride(fz_context *ctx, fz_pixmap *pix);
unsigned char *fz_pixmap_samples(fz_context *ctx, fz_pixmap *pix);
fz_irect fz_pixmap_bbox(fz_context *ctx, fz_pixmap *pix);

// ============================================================================
// Pixmap manipulation
// ============================================================================

void fz_clear_pixmap(fz_context *ctx, fz_pixmap *pix);
void fz_clear_pixmap_with_value(fz_context *ctx, fz_pixmap *pix, int value);
void fz_clear_pixmap_rect_with_value(fz_context *ctx, fz_pixmap *pix, int value, fz_irect r);

void fz_fill_pixmap_with_color(fz_context *ctx, fz_pixmap *pix, fz_colorspace *cs, float *color, fz_color_params color_params);

void fz_invert_pixmap(fz_context *ctx, fz_pixmap *pix);
void fz_invert_pixmap_luminance(fz_context *ctx, fz_pixmap *pix);
void fz_tint_pixmap(fz_context *ctx, fz_pixmap *pix, int black, int white);
void fz_gamma_pixmap(fz_context *ctx, fz_pixmap *pix, float gamma);

void fz_copy_pixmap_rect(fz_context *ctx, fz_pixmap *dest, fz_pixmap *src, fz_irect r, const fz_default_colorspaces *default_cs);
void fz_copy_pixmap_area_converting_seps(fz_context *ctx, fz_pixmap *dst, fz_pixmap *src, fz_colorspace *prf, fz_default_colorspaces *default_cs, fz_color_params color_params, fz_irect bbox);

fz_pixmap *fz_convert_pixmap(fz_context *ctx, fz_pixmap *pix, fz_colorspace *cs, fz_colorspace *prf, fz_default_colorspaces *default_cs, fz_color_params color_params, int keep_alpha);

// ============================================================================
// Scaling
// ============================================================================

fz_pixmap *fz_scale_pixmap(fz_context *ctx, fz_pixmap *src, float x, float y, float w, float h, const fz_irect *clip);
fz_pixmap *fz_scale_pixmap_cached(fz_context *ctx, const fz_pixmap *src, float x, float y, float w, float h, const fz_irect *clip, fz_scale_cache *cache_x, fz_scale_cache *cache_y);

typedef struct fz_scale_cache fz_scale_cache;

fz_scale_cache *fz_new_scale_cache(fz_context *ctx);
void fz_drop_scale_cache(fz_context *ctx, fz_scale_cache *cache);

// ============================================================================
// Subsample
// ============================================================================

fz_pixmap *fz_subsample_pixmap(fz_context *ctx, fz_pixmap *tile, int factor);

// ============================================================================
// Alpha operations
// ============================================================================

void fz_premultiply_pixmap(fz_context *ctx, fz_pixmap *pix);
void fz_unpremultiply_pixmap(fz_context *ctx, fz_pixmap *pix);

// ============================================================================
// Pixel access
// ============================================================================

unsigned char *fz_pixmap_sample_ptr(fz_context *ctx, fz_pixmap *pix, int x, int y);

static inline void fz_set_pixmap_sample(fz_context *ctx, fz_pixmap *pix, int x, int y, int n, unsigned char v)
{
    unsigned char *s = fz_pixmap_sample_ptr(ctx, pix, x, y);
    if (s) s[n] = v;
}

static inline unsigned char fz_get_pixmap_sample(fz_context *ctx, fz_pixmap *pix, int x, int y, int n)
{
    unsigned char *s = fz_pixmap_sample_ptr(ctx, pix, x, y);
    return s ? s[n] : 0;
}

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_PIXMAP_H */

