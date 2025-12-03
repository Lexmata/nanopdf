// NanoPDF - MuPDF API Compatible C Header
// This file provides 100% API compatibility with MuPDF's system.h

#ifndef MUPDF_FITZ_SYSTEM_H
#define MUPDF_FITZ_SYSTEM_H

#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdarg.h>
#include <string.h>
#include <math.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Platform detection
// ============================================================================

#if defined(_WIN32) || defined(_WIN64)
#define FZ_WINDOWS 1
#elif defined(__APPLE__)
#define FZ_MACOS 1
#elif defined(__linux__)
#define FZ_LINUX 1
#endif

// ============================================================================
// Export/Import macros
// ============================================================================

#ifdef FZ_DLL
#ifdef FZ_DLL_EXPORTS
#define FZ_FUNCTION __declspec(dllexport)
#define FZ_DATA __declspec(dllexport)
#else
#define FZ_FUNCTION __declspec(dllimport)
#define FZ_DATA __declspec(dllimport)
#endif
#else
#define FZ_FUNCTION
#define FZ_DATA extern
#endif

// ============================================================================
// Compiler attributes
// ============================================================================

#ifdef __GNUC__
#define FZ_NORETURN __attribute__((noreturn))
#define FZ_PRINTFLIKE(F,V) __attribute__((format(printf, F, V)))
#define FZ_UNUSED __attribute__((unused))
#elif defined(_MSC_VER)
#define FZ_NORETURN __declspec(noreturn)
#define FZ_PRINTFLIKE(F,V)
#define FZ_UNUSED
#else
#define FZ_NORETURN
#define FZ_PRINTFLIKE(F,V)
#define FZ_UNUSED
#endif

// ============================================================================
// Integer types
// ============================================================================

#ifndef TRUE
#define TRUE 1
#endif

#ifndef FALSE
#define FALSE 0
#endif

#ifndef MIN
#define MIN(a,b) ((a) < (b) ? (a) : (b))
#endif

#ifndef MAX
#define MAX(a,b) ((a) > (b) ? (a) : (b))
#endif

#define nelem(x) (sizeof(x) / sizeof((x)[0]))

// ============================================================================
// String functions
// ============================================================================

char *fz_strsep(char **stringp, const char *delim);
size_t fz_strlcpy(char *dst, const char *src, size_t n);
size_t fz_strlcat(char *dst, const char *src, size_t n);
void *fz_memmem(const void *haystack, size_t haystacklen, const void *needle, size_t needlelen);
int fz_strcasecmp(const char *a, const char *b);
int fz_strncasecmp(const char *a, const char *b, size_t n);

// ============================================================================
// Printf variants
// ============================================================================

size_t fz_vsnprintf(char *buffer, size_t space, const char *fmt, va_list args);
size_t fz_snprintf(char *buffer, size_t space, const char *fmt, ...);
char *fz_asprintf(fz_context *ctx, const char *fmt, ...);

// ============================================================================
// Time functions
// ============================================================================

int64_t fz_time(void);

#ifdef __cplusplus
}
#endif

#endif /* MUPDF_FITZ_SYSTEM_H */

