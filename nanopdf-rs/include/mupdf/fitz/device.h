// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's device.h

#ifndef MUPDF_FITZ_DEVICE_H
#define MUPDF_FITZ_DEVICE_H

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

typedef struct fz_device fz_device;
typedef struct fz_path fz_path;
typedef struct fz_stroke_state fz_stroke_state;
typedef struct fz_text fz_text;
typedef struct fz_image fz_image;
typedef struct fz_shade fz_shade;

// ============================================================================
// Blend modes
// ============================================================================

enum {
    FZ_BLEND_NORMAL,
    FZ_BLEND_MULTIPLY,
    FZ_BLEND_SCREEN,
    FZ_BLEND_OVERLAY,
    FZ_BLEND_DARKEN,
    FZ_BLEND_LIGHTEN,
    FZ_BLEND_COLOR_DODGE,
    FZ_BLEND_COLOR_BURN,
    FZ_BLEND_HARD_LIGHT,
    FZ_BLEND_SOFT_LIGHT,
    FZ_BLEND_DIFFERENCE,
    FZ_BLEND_EXCLUSION,
    FZ_BLEND_HUE,
    FZ_BLEND_SATURATION,
    FZ_BLEND_COLOR,
    FZ_BLEND_LUMINOSITY,
    FZ_BLEND_MODEMASK = 15,
    FZ_BLEND_ISOLATED = 16,
    FZ_BLEND_KNOCKOUT = 32,
};

const char *fz_blendmode_name(int blendmode);
int fz_lookup_blendmode(const char *name);

// ============================================================================
// Line cap/join
// ============================================================================

enum {
    FZ_LINECAP_BUTT = 0,
    FZ_LINECAP_ROUND = 1,
    FZ_LINECAP_SQUARE = 2,
    FZ_LINECAP_TRIANGLE = 3,
};

enum {
    FZ_LINEJOIN_MITER = 0,
    FZ_LINEJOIN_ROUND = 1,
    FZ_LINEJOIN_BEVEL = 2,
    FZ_LINEJOIN_MITER_XPS = 3,
};

// ============================================================================
// Stroke state
// ============================================================================

struct fz_stroke_state {
    int refs;
    int start_cap, dash_cap, end_cap;
    int linejoin;
    float linewidth;
    float miterlimit;
    float dash_phase;
    int dash_len;
    float dash_list[32];
};

fz_stroke_state *fz_new_stroke_state(fz_context *ctx);
fz_stroke_state *fz_new_stroke_state_with_dash_len(fz_context *ctx, int len);
fz_stroke_state *fz_keep_stroke_state(fz_context *ctx, fz_stroke_state *stroke);
void fz_drop_stroke_state(fz_context *ctx, fz_stroke_state *stroke);
fz_stroke_state *fz_unshare_stroke_state(fz_context *ctx, fz_stroke_state *shared);
fz_stroke_state *fz_unshare_stroke_state_with_dash_len(fz_context *ctx, fz_stroke_state *shared, int len);
fz_stroke_state *fz_clone_stroke_state(fz_context *ctx, fz_stroke_state *stroke);

extern const fz_stroke_state fz_default_stroke_state;

// ============================================================================
// Device reference counting
// ============================================================================

fz_device *fz_keep_device(fz_context *ctx, fz_device *dev);
void fz_drop_device(fz_context *ctx, fz_device *dev);

// ============================================================================
// Device hint flags
// ============================================================================

enum {
    FZ_DEVFLAG_MASK = 1,
    FZ_DEVFLAG_COLOR = 2,
    FZ_DEVFLAG_UNCACHEABLE = 4,
    FZ_DEVFLAG_FILLCOLOR_UNDEFINED = 8,
    FZ_DEVFLAG_STROKECOLOR_UNDEFINED = 16,
    FZ_DEVFLAG_STARTCAP_UNDEFINED = 32,
    FZ_DEVFLAG_DASHCAP_UNDEFINED = 64,
    FZ_DEVFLAG_ENDCAP_UNDEFINED = 128,
    FZ_DEVFLAG_LINEJOIN_UNDEFINED = 256,
    FZ_DEVFLAG_MITERLIMIT_UNDEFINED = 512,
    FZ_DEVFLAG_LINEWIDTH_UNDEFINED = 1024,
    FZ_DEVFLAG_BBOX_DEFINED = 2048,
    FZ_DEVFLAG_GRIDFIT_AS_TILED = 4096,
};

// ============================================================================
// Device operations
// ============================================================================

void fz_close_device(fz_context *ctx, fz_device *dev);
void fz_enable_device_hints(fz_context *ctx, fz_device *dev, int hints);
void fz_disable_device_hints(fz_context *ctx, fz_device *dev, int hints);

void fz_fill_path(fz_context *ctx, fz_device *dev, const fz_path *path, int even_odd, fz_matrix ctm, fz_colorspace *colorspace, const float *color, float alpha, fz_color_params color_params);
void fz_stroke_path(fz_context *ctx, fz_device *dev, const fz_path *path, const fz_stroke_state *stroke, fz_matrix ctm, fz_colorspace *colorspace, const float *color, float alpha, fz_color_params color_params);
void fz_clip_path(fz_context *ctx, fz_device *dev, const fz_path *path, int even_odd, fz_matrix ctm, fz_rect scissor);
void fz_clip_stroke_path(fz_context *ctx, fz_device *dev, const fz_path *path, const fz_stroke_state *stroke, fz_matrix ctm, fz_rect scissor);

void fz_fill_text(fz_context *ctx, fz_device *dev, const fz_text *text, fz_matrix ctm, fz_colorspace *colorspace, const float *color, float alpha, fz_color_params color_params);
void fz_stroke_text(fz_context *ctx, fz_device *dev, const fz_text *text, const fz_stroke_state *stroke, fz_matrix ctm, fz_colorspace *colorspace, const float *color, float alpha, fz_color_params color_params);
void fz_clip_text(fz_context *ctx, fz_device *dev, const fz_text *text, fz_matrix ctm, fz_rect scissor);
void fz_clip_stroke_text(fz_context *ctx, fz_device *dev, const fz_text *text, const fz_stroke_state *stroke, fz_matrix ctm, fz_rect scissor);
void fz_ignore_text(fz_context *ctx, fz_device *dev, const fz_text *text, fz_matrix ctm);

void fz_fill_shade(fz_context *ctx, fz_device *dev, fz_shade *shade, fz_matrix ctm, float alpha, fz_color_params color_params);
void fz_fill_image(fz_context *ctx, fz_device *dev, fz_image *image, fz_matrix ctm, float alpha, fz_color_params color_params);
void fz_fill_image_mask(fz_context *ctx, fz_device *dev, fz_image *image, fz_matrix ctm, fz_colorspace *colorspace, const float *color, float alpha, fz_color_params color_params);
void fz_clip_image_mask(fz_context *ctx, fz_device *dev, fz_image *image, fz_matrix ctm, fz_rect scissor);

void fz_pop_clip(fz_context *ctx, fz_device *dev);

void fz_begin_mask(fz_context *ctx, fz_device *dev, fz_rect area, int luminosity, fz_colorspace *colorspace, const float *bc, fz_color_params color_params);
void fz_end_mask(fz_context *ctx, fz_device *dev);

void fz_begin_group(fz_context *ctx, fz_device *dev, fz_rect area, fz_colorspace *cs, int isolated, int knockout, int blendmode, float alpha);
void fz_end_group(fz_context *ctx, fz_device *dev);

void fz_begin_tile(fz_context *ctx, fz_device *dev, fz_rect area, fz_rect view, float xstep, float ystep, fz_matrix ctm);
void fz_begin_tile_id(fz_context *ctx, fz_device *dev, fz_rect area, fz_rect view, float xstep, float ystep, fz_matrix ctm, int id);
void fz_end_tile(fz_context *ctx, fz_device *dev);

void fz_render_flags(fz_context *ctx, fz_device *dev, int set, int clear);
void fz_set_default_colorspaces(fz_context *ctx, fz_device *dev, fz_default_colorspaces *default_cs);

void fz_begin_layer(fz_context *ctx, fz_device *dev, const char *layer_name);
void fz_end_layer(fz_context *ctx, fz_device *dev);

// ============================================================================
// Device creation
// ============================================================================

fz_device *fz_new_draw_device(fz_context *ctx, fz_matrix transform, fz_pixmap *dest);
fz_device *fz_new_draw_device_with_bbox(fz_context *ctx, fz_matrix transform, fz_pixmap *dest, const fz_irect *clip);
fz_device *fz_new_draw_device_with_proof(fz_context *ctx, fz_matrix transform, fz_pixmap *dest, fz_colorspace *proof_cs);

fz_device *fz_new_trace_device(fz_context *ctx, fz_output *out);
fz_device *fz_new_bbox_device(fz_context *ctx, fz_rect *rectp);
fz_device *fz_new_test_device(fz_context *ctx, int *is_color, float threshold, int options, fz_device *passthrough);

typedef struct fz_display_list fz_display_list;

fz_display_list *fz_new_display_list(fz_context *ctx, fz_rect mediabox);
fz_display_list *fz_keep_display_list(fz_context *ctx, fz_display_list *list);
void fz_drop_display_list(fz_context *ctx, fz_display_list *list);
fz_device *fz_new_list_device(fz_context *ctx, fz_display_list *list);
void fz_run_display_list(fz_context *ctx, fz_display_list *list, fz_device *dev, fz_matrix ctm, fz_rect scissor, fz_cookie *cookie);
fz_rect fz_bound_display_list(fz_context *ctx, fz_display_list *list);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_DEVICE_H */

