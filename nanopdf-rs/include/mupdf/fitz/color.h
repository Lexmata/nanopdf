// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's color.h

#ifndef MUPDF_FITZ_COLOR_H
#define MUPDF_FITZ_COLOR_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Types
// ============================================================================

typedef struct fz_colorspace fz_colorspace;
typedef struct fz_default_colorspaces fz_default_colorspaces;

// ============================================================================
// Color parameters
// ============================================================================

typedef struct {
    uint8_t ri;  /* Rendering intent */
    uint8_t bp;  /* Black point compensation */
    uint8_t op;  /* Overprint */
    uint8_t opm; /* Overprint mode */
} fz_color_params;

extern const fz_color_params fz_default_color_params;

enum {
    FZ_RI_PERCEPTUAL,
    FZ_RI_RELATIVE_COLORIMETRIC,
    FZ_RI_SATURATION,
    FZ_RI_ABSOLUTE_COLORIMETRIC,
};

// ============================================================================
// Colorspace types
// ============================================================================

enum fz_colorspace_type {
    FZ_COLORSPACE_NONE,
    FZ_COLORSPACE_GRAY,
    FZ_COLORSPACE_RGB,
    FZ_COLORSPACE_BGR,
    FZ_COLORSPACE_CMYK,
    FZ_COLORSPACE_LAB,
    FZ_COLORSPACE_INDEXED,
    FZ_COLORSPACE_SEPARATION,
};

// ============================================================================
// Device colorspaces
// ============================================================================

fz_colorspace *fz_device_gray(fz_context *ctx);
fz_colorspace *fz_device_rgb(fz_context *ctx);
fz_colorspace *fz_device_bgr(fz_context *ctx);
fz_colorspace *fz_device_cmyk(fz_context *ctx);
fz_colorspace *fz_device_lab(fz_context *ctx);

// ============================================================================
// Colorspace reference counting
// ============================================================================

fz_colorspace *fz_keep_colorspace(fz_context *ctx, fz_colorspace *colorspace);
void fz_drop_colorspace(fz_context *ctx, fz_colorspace *colorspace);

// ============================================================================
// Colorspace properties
// ============================================================================

const char *fz_colorspace_name(fz_context *ctx, fz_colorspace *colorspace);
enum fz_colorspace_type fz_colorspace_type(fz_context *ctx, fz_colorspace *colorspace);
int fz_colorspace_n(fz_context *ctx, fz_colorspace *colorspace);
int fz_colorspace_is_gray(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_rgb(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_cmyk(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_lab(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_indexed(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_device_n(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_subtractive(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_device(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_device_gray(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_is_device_cmyk(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_device_n_has_only_cmyk(fz_context *ctx, fz_colorspace *cs);
int fz_colorspace_device_n_has_cmyk(fz_context *ctx, fz_colorspace *cs);

// ============================================================================
// Color conversion
// ============================================================================

void fz_convert_color(fz_context *ctx, fz_colorspace *ss, const float *sv, fz_colorspace *ds, float *dv, fz_colorspace *is, fz_color_params params);

// ============================================================================
// Default colorspaces
// ============================================================================

fz_default_colorspaces *fz_new_default_colorspaces(fz_context *ctx);
fz_default_colorspaces *fz_keep_default_colorspaces(fz_context *ctx, fz_default_colorspaces *default_cs);
void fz_drop_default_colorspaces(fz_context *ctx, fz_default_colorspaces *default_cs);
fz_default_colorspaces *fz_clone_default_colorspaces(fz_context *ctx, fz_default_colorspaces *base);

fz_colorspace *fz_default_gray(fz_context *ctx, fz_default_colorspaces *default_cs);
fz_colorspace *fz_default_rgb(fz_context *ctx, fz_default_colorspaces *default_cs);
fz_colorspace *fz_default_cmyk(fz_context *ctx, fz_default_colorspaces *default_cs);
fz_colorspace *fz_default_output_intent(fz_context *ctx, fz_default_colorspaces *default_cs);

void fz_set_default_gray(fz_context *ctx, fz_default_colorspaces *default_cs, fz_colorspace *cs);
void fz_set_default_rgb(fz_context *ctx, fz_default_colorspaces *default_cs, fz_colorspace *cs);
void fz_set_default_cmyk(fz_context *ctx, fz_default_colorspaces *default_cs, fz_colorspace *cs);
void fz_set_default_output_intent(fz_context *ctx, fz_default_colorspaces *default_cs, fz_colorspace *cs);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_COLOR_H */

