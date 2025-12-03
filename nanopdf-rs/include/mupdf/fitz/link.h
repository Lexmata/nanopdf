// NanoPDF - MuPDF API Compatible C Header

#ifndef MUPDF_FITZ_LINK_H
#define MUPDF_FITZ_LINK_H

#include "mupdf/fitz/system.h"
#include "mupdf/fitz/context.h"
#include "mupdf/fitz/geometry.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct fz_link fz_link;

struct fz_link {
    int refs;
    fz_link *next;
    fz_rect rect;
    char *uri;
};

fz_link *fz_new_link(fz_context *ctx, fz_rect bbox, const char *uri);
fz_link *fz_keep_link(fz_context *ctx, fz_link *link);
void fz_drop_link(fz_context *ctx, fz_link *link);

int fz_is_external_link(fz_context *ctx, const char *uri);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_LINK_H */

