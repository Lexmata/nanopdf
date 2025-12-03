// NanoPDF - MuPDF API Compatible C Header

#ifndef MUPDF_FITZ_OUTLINE_H
#define MUPDF_FITZ_OUTLINE_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct fz_outline fz_outline;

struct fz_outline {
    int refs;
    char *title;
    char *uri;
    int page;
    float x, y;
    fz_outline *next;
    fz_outline *down;
    int is_open;
};

void fz_drop_outline(fz_context *ctx, fz_outline *outline);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_OUTLINE_H */

