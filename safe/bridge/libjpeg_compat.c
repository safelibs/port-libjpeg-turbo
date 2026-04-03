/*
 * Temporary ABI bridge for the bootstrap phase.
 *
 * The Rust port still stages upstream C objects in this phase, but Ubuntu's
 * Debian symbol manifests include libjpeg v8 compatibility entry points that
 * are not emitted by the stock shared library build on amd64.  Keep those
 * exports available here until the Rust port provides native implementations.
 */

#define JPEG_INTERNALS
#include "jinclude.h"
#include "jpeglib.h"
#include "jdct.h"

#include <string.h>

#define JPEG_COMPAT_BLOCK_VALUES  (DCTSIZE2 + 16)
#define JPEG_COMPAT_MAX_BLOCK     16

void *auxv = NULL;

static void
jpeg_fdct_rect_bridge(DCTELEM *data, int width, int height)
{
  DCTELEM tmp[DCTSIZE2];
  int copy_width = width < DCTSIZE ? width : DCTSIZE;
  int copy_height = height < DCTSIZE ? height : DCTSIZE;
  int row;

  memset(tmp, 0, sizeof(tmp));
  for (row = 0; row < copy_height; row++) {
    memcpy(&tmp[row * DCTSIZE], &data[row * width],
           (size_t)copy_width * sizeof(DCTELEM));
  }

  jpeg_fdct_islow(tmp);

  memset(data, 0, (size_t)width * (size_t)height * sizeof(DCTELEM));
  for (row = 0; row < copy_height; row++) {
    memcpy(&data[row * width], &tmp[row * DCTSIZE],
           (size_t)copy_width * sizeof(DCTELEM));
  }
}

typedef void (*jpeg_idct_square_fn)(j_decompress_ptr cinfo,
                                    jpeg_component_info *compptr,
                                    JCOEFPTR coef_block,
                                    JSAMPARRAY output_buf,
                                    JDIMENSION output_col);

static jpeg_idct_square_fn
jpeg_idct_square_dispatch(int size)
{
  switch (size) {
  case 1:
    return jpeg_idct_1x1;
  case 2:
    return jpeg_idct_2x2;
  case 3:
    return jpeg_idct_3x3;
  case 4:
    return jpeg_idct_4x4;
  case 5:
    return jpeg_idct_5x5;
  case 6:
    return jpeg_idct_6x6;
  case 7:
    return jpeg_idct_7x7;
  case 8:
    return jpeg_idct_islow;
  case 9:
    return jpeg_idct_9x9;
  case 10:
    return jpeg_idct_10x10;
  case 11:
    return jpeg_idct_11x11;
  case 12:
    return jpeg_idct_12x12;
  case 13:
    return jpeg_idct_13x13;
  case 14:
    return jpeg_idct_14x14;
  case 15:
    return jpeg_idct_15x15;
  case 16:
    return jpeg_idct_16x16;
  default:
    return jpeg_idct_islow;
  }
}

static void
jpeg_idct_rect_bridge(j_decompress_ptr cinfo, jpeg_component_info *compptr,
                      JCOEFPTR coef_block, JSAMPARRAY output_buf,
                      JDIMENSION output_col, int width, int height)
{
  JSAMPLE scratch[JPEG_COMPAT_MAX_BLOCK][JPEG_COMPAT_MAX_BLOCK];
  JSAMPROW rows[JPEG_COMPAT_MAX_BLOCK];
  jpeg_idct_square_fn decode;
  int size = width > height ? width : height;
  int row;

  decode = jpeg_idct_square_dispatch(size);
  memset(scratch, 0, sizeof(scratch));
  for (row = 0; row < JPEG_COMPAT_MAX_BLOCK; row++)
    rows[row] = scratch[row];

  decode(cinfo, compptr, coef_block, rows, 0);

  for (row = 0; row < height; row++) {
    memcpy(output_buf[row] + output_col, scratch[row],
           (size_t)width * sizeof(JSAMPLE));
  }
}

#define JPEG_DEFINE_FDCT_BRIDGE(width, height) \
  void jpeg_fdct_##width##x##height(DCTELEM *data) \
  { \
    jpeg_fdct_rect_bridge(data, width, height); \
  }

JPEG_DEFINE_FDCT_BRIDGE(1, 1)
JPEG_DEFINE_FDCT_BRIDGE(1, 2)
JPEG_DEFINE_FDCT_BRIDGE(2, 1)
JPEG_DEFINE_FDCT_BRIDGE(2, 2)
JPEG_DEFINE_FDCT_BRIDGE(2, 4)
JPEG_DEFINE_FDCT_BRIDGE(3, 3)
JPEG_DEFINE_FDCT_BRIDGE(3, 6)
JPEG_DEFINE_FDCT_BRIDGE(4, 2)
JPEG_DEFINE_FDCT_BRIDGE(4, 4)
JPEG_DEFINE_FDCT_BRIDGE(4, 8)
JPEG_DEFINE_FDCT_BRIDGE(5, 5)
JPEG_DEFINE_FDCT_BRIDGE(5, 10)
JPEG_DEFINE_FDCT_BRIDGE(6, 3)
JPEG_DEFINE_FDCT_BRIDGE(6, 6)
JPEG_DEFINE_FDCT_BRIDGE(6, 12)
JPEG_DEFINE_FDCT_BRIDGE(7, 7)
JPEG_DEFINE_FDCT_BRIDGE(7, 14)
JPEG_DEFINE_FDCT_BRIDGE(8, 4)
JPEG_DEFINE_FDCT_BRIDGE(8, 16)
JPEG_DEFINE_FDCT_BRIDGE(9, 9)
JPEG_DEFINE_FDCT_BRIDGE(10, 5)
JPEG_DEFINE_FDCT_BRIDGE(10, 10)
JPEG_DEFINE_FDCT_BRIDGE(11, 11)
JPEG_DEFINE_FDCT_BRIDGE(12, 6)
JPEG_DEFINE_FDCT_BRIDGE(12, 12)
JPEG_DEFINE_FDCT_BRIDGE(13, 13)
JPEG_DEFINE_FDCT_BRIDGE(14, 7)
JPEG_DEFINE_FDCT_BRIDGE(14, 14)
JPEG_DEFINE_FDCT_BRIDGE(15, 15)
JPEG_DEFINE_FDCT_BRIDGE(16, 8)
JPEG_DEFINE_FDCT_BRIDGE(16, 16)

#define JPEG_DEFINE_IDCT_BRIDGE(width, height) \
  void jpeg_idct_##width##x##height(j_decompress_ptr cinfo, \
                                    jpeg_component_info *compptr, \
                                    JCOEFPTR coef_block, \
                                    JSAMPARRAY output_buf, \
                                    JDIMENSION output_col) \
  { \
    jpeg_idct_rect_bridge(cinfo, compptr, coef_block, output_buf, output_col, \
                          width, height); \
  }

JPEG_DEFINE_IDCT_BRIDGE(1, 2)
JPEG_DEFINE_IDCT_BRIDGE(2, 1)
JPEG_DEFINE_IDCT_BRIDGE(2, 4)
JPEG_DEFINE_IDCT_BRIDGE(3, 6)
JPEG_DEFINE_IDCT_BRIDGE(4, 2)
JPEG_DEFINE_IDCT_BRIDGE(4, 8)
JPEG_DEFINE_IDCT_BRIDGE(5, 10)
JPEG_DEFINE_IDCT_BRIDGE(6, 3)
JPEG_DEFINE_IDCT_BRIDGE(6, 12)
JPEG_DEFINE_IDCT_BRIDGE(7, 14)
JPEG_DEFINE_IDCT_BRIDGE(8, 4)
JPEG_DEFINE_IDCT_BRIDGE(8, 16)
JPEG_DEFINE_IDCT_BRIDGE(10, 5)
JPEG_DEFINE_IDCT_BRIDGE(12, 6)
JPEG_DEFINE_IDCT_BRIDGE(14, 7)
JPEG_DEFINE_IDCT_BRIDGE(16, 8)

int
libjpeg_general_init(void)
{
  return 0;
}
