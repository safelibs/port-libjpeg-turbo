#include <setjmp.h>
#include <stddef.h>
#include <stdio.h>
#include <string.h>

#include "jpeglib.h"
#include "jerror.h"

void jpeg_rs_invoke_error_exit(j_common_ptr cinfo) {
  (*cinfo->err->error_exit)(cinfo);
}

typedef struct {
  struct jpeg_error_mgr pub;
  jmp_buf jb;
  char message[JMSG_LENGTH_MAX];
} jpeg_rs_test_error_mgr;

static void jpeg_rs_test_error_exit(j_common_ptr cinfo) {
  jpeg_rs_test_error_mgr *err = (jpeg_rs_test_error_mgr *)cinfo->err;
  (*cinfo->err->format_message)(cinfo, err->message);
  longjmp(err->jb, 1);
}

static void jpeg_rs_copy_message(char *dst, size_t dst_len,
                                 const jpeg_rs_test_error_mgr *err) {
  if (!dst || dst_len == 0)
    return;
  strncpy(dst, err->message, dst_len);
  dst[dst_len - 1] = '\0';
}

int jpeg_rs_expect_mem_src_error(char *message, size_t message_len) {
  struct jpeg_decompress_struct cinfo;
  jpeg_rs_test_error_mgr err;

  memset(&cinfo, 0, sizeof(cinfo));
  memset(&err, 0, sizeof(err));
  cinfo.err = jpeg_std_error(&err.pub);
  err.pub.error_exit = jpeg_rs_test_error_exit;

  if (setjmp(err.jb)) {
    jpeg_rs_copy_message(message, message_len, &err);
    jpeg_destroy_decompress(&cinfo);
    return 1;
  }

  jpeg_CreateDecompress(&cinfo, JPEG_LIB_VERSION,
                        sizeof(struct jpeg_decompress_struct));
  jpeg_mem_src(&cinfo, NULL, 0);
  jpeg_destroy_decompress(&cinfo);
  return 0;
}

int jpeg_rs_probe_default_colorspace(int color_space, char *message,
                                     size_t message_len) {
  struct jpeg_compress_struct cinfo;
  jpeg_rs_test_error_mgr err;

  memset(&cinfo, 0, sizeof(cinfo));
  memset(&err, 0, sizeof(err));
  cinfo.err = jpeg_std_error(&err.pub);
  err.pub.error_exit = jpeg_rs_test_error_exit;

  if (setjmp(err.jb)) {
    jpeg_rs_copy_message(message, message_len, &err);
    jpeg_destroy_compress(&cinfo);
    return 0;
  }

  jpeg_CreateCompress(&cinfo, JPEG_LIB_VERSION,
                      sizeof(struct jpeg_compress_struct));
  cinfo.input_components = 3;
  jpeg_set_defaults(&cinfo);
  cinfo.in_color_space = color_space;
  jpeg_default_colorspace(&cinfo);
  jpeg_destroy_compress(&cinfo);
  return 1;
}

int jpeg_rs_read_header_info(const unsigned char *jpeg_buf,
                             unsigned long jpeg_size, int *progressive_mode,
                             int *arith_code, unsigned int *restart_interval,
                             char *message, size_t message_len) {
  struct jpeg_decompress_struct cinfo;
  jpeg_rs_test_error_mgr err;

  memset(&cinfo, 0, sizeof(cinfo));
  memset(&err, 0, sizeof(err));
  if (progressive_mode)
    *progressive_mode = 0;
  if (arith_code)
    *arith_code = 0;
  if (restart_interval)
    *restart_interval = 0;

  cinfo.err = jpeg_std_error(&err.pub);
  err.pub.error_exit = jpeg_rs_test_error_exit;

  if (setjmp(err.jb)) {
    jpeg_rs_copy_message(message, message_len, &err);
    jpeg_destroy_decompress(&cinfo);
    return -1;
  }

  jpeg_CreateDecompress(&cinfo, JPEG_LIB_VERSION,
                        sizeof(struct jpeg_decompress_struct));
  jpeg_mem_src(&cinfo, jpeg_buf, jpeg_size);
  jpeg_read_header(&cinfo, TRUE);

  if (progressive_mode)
    *progressive_mode = cinfo.progressive_mode ? 1 : 0;
  if (arith_code)
    *arith_code = cinfo.arith_code ? 1 : 0;
  if (restart_interval)
    *restart_interval = cinfo.restart_interval;

  jpeg_destroy_decompress(&cinfo);
  return 0;
}
