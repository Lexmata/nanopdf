// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's version.h

#ifndef MUPDF_FITZ_VERSION_H
#define MUPDF_FITZ_VERSION_H

// NanoPDF version (MuPDF API compatible)
#define FZ_VERSION "0.1.0"
#define FZ_VERSION_MAJOR 0
#define FZ_VERSION_MINOR 1
#define FZ_VERSION_PATCH 0

// For compatibility, also provide mupdf version macros
#define MUPDF_VERSION "1.26.12-nanopdf"

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Return the version string.
 */
const char *fz_version(void);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_VERSION_H */

