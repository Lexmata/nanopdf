// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's geometry.h

#ifndef MUPDF_FITZ_GEOMETRY_H
#define MUPDF_FITZ_GEOMETRY_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

#ifndef M_PI
#define M_PI 3.14159265358979323846
#endif

#define FZ_MIN_INF_RECT ((int)0x80000000)
#define FZ_MAX_INF_RECT ((int)0x7fffff80)

// ============================================================================
// Types
// ============================================================================

typedef struct {
    float x, y;
} fz_point;

typedef struct {
    float x0, y0;
    float x1, y1;
} fz_rect;

typedef struct {
    int x0, y0;
    int x1, y1;
} fz_irect;

typedef struct {
    float a, b, c, d, e, f;
} fz_matrix;

typedef struct {
    fz_point ul, ur, ll, lr;
} fz_quad;

// ============================================================================
// Constants
// ============================================================================

extern const fz_rect fz_unit_rect;
extern const fz_rect fz_empty_rect;
extern const fz_rect fz_infinite_rect;
extern const fz_rect fz_invalid_rect;

extern const fz_irect fz_empty_irect;
extern const fz_irect fz_infinite_irect;
extern const fz_irect fz_invalid_irect;

extern const fz_matrix fz_identity;

extern const fz_quad fz_invalid_quad;
extern const fz_quad fz_infinite_quad;

// ============================================================================
// Inline constructors
// ============================================================================

static inline fz_point fz_make_point(float x, float y) {
    fz_point p = { x, y };
    return p;
}

static inline fz_rect fz_make_rect(float x0, float y0, float x1, float y1) {
    fz_rect r = { x0, y0, x1, y1 };
    return r;
}

static inline fz_irect fz_make_irect(int x0, int y0, int x1, int y1) {
    fz_irect r = { x0, y0, x1, y1 };
    return r;
}

static inline fz_matrix fz_make_matrix(float a, float b, float c, float d, float e, float f) {
    fz_matrix m = { a, b, c, d, e, f };
    return m;
}

static inline fz_quad fz_make_quad(
    float ul_x, float ul_y,
    float ur_x, float ur_y,
    float ll_x, float ll_y,
    float lr_x, float lr_y)
{
    fz_quad q = {
        { ul_x, ul_y },
        { ur_x, ur_y },
        { ll_x, ll_y },
        { lr_x, lr_y },
    };
    return q;
}

// ============================================================================
// Rect functions
// ============================================================================

static inline int fz_is_empty_rect(fz_rect r) {
    return (r.x0 >= r.x1 || r.y0 >= r.y1);
}

static inline int fz_is_empty_irect(fz_irect r) {
    return (r.x0 >= r.x1 || r.y0 >= r.y1);
}

static inline int fz_is_infinite_rect(fz_rect r) {
    return (r.x0 == FZ_MIN_INF_RECT && r.x1 == FZ_MAX_INF_RECT &&
            r.y0 == FZ_MIN_INF_RECT && r.y1 == FZ_MAX_INF_RECT);
}

static inline int fz_is_infinite_irect(fz_irect r) {
    return (r.x0 == FZ_MIN_INF_RECT && r.x1 == FZ_MAX_INF_RECT &&
            r.y0 == FZ_MIN_INF_RECT && r.y1 == FZ_MAX_INF_RECT);
}

static inline int fz_is_valid_rect(fz_rect r) {
    return (r.x0 <= r.x1 && r.y0 <= r.y1);
}

static inline int fz_is_valid_irect(fz_irect r) {
    return (r.x0 <= r.x1 && r.y0 <= r.y1);
}

static inline unsigned int fz_irect_width(fz_irect r) {
    if (r.x0 >= r.x1) return 0;
    return (unsigned int)(r.x1 - r.x0);
}

static inline int fz_irect_height(fz_irect r) {
    if (r.y0 >= r.y1) return 0;
    return (int)(r.y1 - r.y0);
}

static inline int fz_is_identity(fz_matrix m) {
    return m.a == 1 && m.b == 0 && m.c == 0 && m.d == 1 && m.e == 0 && m.f == 0;
}

// ============================================================================
// Matrix functions
// ============================================================================

fz_matrix fz_concat(fz_matrix left, fz_matrix right);
fz_matrix fz_scale(float sx, float sy);
fz_matrix fz_pre_scale(fz_matrix m, float sx, float sy);
fz_matrix fz_post_scale(fz_matrix m, float sx, float sy);
fz_matrix fz_shear(float sx, float sy);
fz_matrix fz_pre_shear(fz_matrix m, float sx, float sy);
fz_matrix fz_rotate(float degrees);
fz_matrix fz_pre_rotate(fz_matrix m, float degrees);
fz_matrix fz_translate(float tx, float ty);
fz_matrix fz_pre_translate(fz_matrix m, float tx, float ty);
fz_matrix fz_transform_page(fz_rect mediabox, float resolution, float rotate);
fz_matrix fz_invert_matrix(fz_matrix matrix);
int fz_try_invert_matrix(fz_matrix *inv, fz_matrix src);
int fz_is_rectilinear(fz_matrix m);
float fz_matrix_expansion(fz_matrix m);
float fz_matrix_max_expansion(fz_matrix m);
fz_matrix fz_gridfit_matrix(int as_tiled, fz_matrix m);

// ============================================================================
// Rect operations
// ============================================================================

fz_rect fz_intersect_rect(fz_rect a, fz_rect b);
fz_irect fz_intersect_irect(fz_irect a, fz_irect b);
fz_rect fz_union_rect(fz_rect a, fz_rect b);
fz_irect fz_irect_from_rect(fz_rect rect);
fz_irect fz_round_rect(fz_rect rect);
fz_rect fz_rect_from_irect(fz_irect bbox);
fz_rect fz_expand_rect(fz_rect b, float expand);
fz_irect fz_expand_irect(fz_irect a, int expand);
fz_rect fz_include_point_in_rect(fz_rect r, fz_point p);
fz_rect fz_translate_rect(fz_rect a, float xoff, float yoff);
fz_irect fz_translate_irect(fz_irect a, int xoff, int yoff);
int fz_contains_rect(fz_rect a, fz_rect b);
int fz_overlaps_rect(fz_rect a, fz_rect b);

// ============================================================================
// Point/transform functions
// ============================================================================

fz_point fz_transform_point(fz_point point, fz_matrix m);
fz_point fz_transform_point_xy(float x, float y, fz_matrix m);
fz_point fz_transform_vector(fz_point vector, fz_matrix m);
fz_rect fz_transform_rect(fz_rect rect, fz_matrix m);
fz_point fz_normalize_vector(fz_point p);

// ============================================================================
// Quad functions
// ============================================================================

int fz_is_valid_quad(fz_quad q);
int fz_is_empty_quad(fz_quad q);
int fz_is_infinite_quad(fz_quad q);
fz_quad fz_quad_from_rect(fz_rect r);
fz_rect fz_rect_from_quad(fz_quad q);
fz_quad fz_transform_quad(fz_quad q, fz_matrix m);
int fz_is_point_inside_quad(fz_point p, fz_quad q);
int fz_is_point_inside_rect(fz_point p, fz_rect r);
int fz_is_point_inside_irect(int x, int y, fz_irect r);
int fz_is_rect_inside_rect(fz_rect inner, fz_rect outer);
int fz_is_irect_inside_irect(fz_irect inner, fz_irect outer);
int fz_is_quad_inside_quad(fz_quad needle, fz_quad haystack);
int fz_is_quad_intersecting_quad(fz_quad a, fz_quad b);

// ============================================================================
// Math utilities
// ============================================================================

float fz_atof(const char *s);
int fz_atoi(const char *s);
int64_t fz_atoi64(const char *s);
size_t fz_atoz(const char *s);

static inline float fz_abs(float f) { return (f < 0 ? -f : f); }
static inline int fz_absi(int i) { return (i < 0 ? -i : i); }
static inline float fz_min(float a, float b) { return (a < b ? a : b); }
static inline int fz_mini(int a, int b) { return (a < b ? a : b); }
static inline size_t fz_minz(size_t a, size_t b) { return (a < b ? a : b); }
static inline int64_t fz_mini64(int64_t a, int64_t b) { return (a < b ? a : b); }
static inline float fz_max(float a, float b) { return (a > b ? a : b); }
static inline int fz_maxi(int a, int b) { return (a > b ? a : b); }
static inline size_t fz_maxz(size_t a, size_t b) { return (a > b ? a : b); }
static inline int64_t fz_maxi64(int64_t a, int64_t b) { return (a > b ? a : b); }
static inline float fz_clamp(float x, float min, float max) { return x < min ? min : x > max ? max : x; }
static inline int fz_clampi(int x, int min, int max) { return x < min ? min : x > max ? max : x; }
static inline int64_t fz_clamp64(int64_t x, int64_t min, int64_t max) { return x < min ? min : x > max ? max : x; }
static inline double fz_clampd(double x, double min, double max) { return x < min ? min : x > max ? max : x; }

static inline int fz_mul255(int a, int b) {
    int x = a * b + 128;
    x += x >> 8;
    return x >> 8;
}

static inline int fz_div255(int c, int a) {
    return a ? c * (255 * 256 / a) >> 8 : 0;
}

#define FZ_EXPAND(A) ((A)+((A)>>7))
#define FZ_COMBINE(A,B) (((A)*(B))>>8)
#define FZ_COMBINE2(A,B,C,D) (((A) * (B) + (C) * (D))>>8)
#define FZ_BLEND(SRC, DST, AMOUNT) ((((SRC)-(DST))*(AMOUNT) + ((DST)<<8))>>8)
#define DIV_BY_ZERO(a, b, min, max) (((a) < 0) ^ ((b) < 0) ? (min) : (max))

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_GEOMETRY_H */

