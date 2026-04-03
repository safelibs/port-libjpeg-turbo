/*
 * Copyright (C)2022, 2026 D. R. Commander.  All Rights Reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * - Redistributions of source code must retain the above copyright notice,
 *   this list of conditions and the following disclaimer.
 * - Redistributions in binary form must reproduce the above copyright notice,
 *   this list of conditions and the following disclaimer in the documentation
 *   and/or other materials provided with the distribution.
 * - Neither the name of the libjpeg-turbo Project nor the names of its
 *   contributors may be used to endorse or promote products derived from this
 *   software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS",
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT HOLDERS OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <jpeglib.h>
#include <jerror.h>
#include "turbojpeg.h"

typedef struct _error_mgr {
  struct jpeg_error_mgr pub;
  jmp_buf jb;
  char message[JMSG_LENGTH_MAX];
} error_mgr;

typedef struct {
  struct jpeg_source_mgr pub;
} memory_src_mgr;

typedef struct {
  int progressive_mode;
  int arith_code;
  unsigned int restart_interval;
} jpeg_header_info;

typedef struct {
  const char *name;
  char *value;
  int was_set;
} env_guard;


static void my_error_exit(j_common_ptr cinfo)
{
  error_mgr *myerr = (error_mgr *)cinfo->err;

  (*cinfo->err->format_message)(cinfo, myerr->message);
  longjmp(myerr->jb, 1);
}

static char *dup_string(const char *src)
{
  size_t len;
  char *dst;

  if (!src)
    return NULL;
  len = strlen(src) + 1;
  dst = (char *)malloc(len);
  if (!dst)
    return NULL;
  memcpy(dst, src, len);
  return dst;
}

static int save_env(env_guard *guard, const char *name)
{
  const char *value = getenv(name);

  guard->name = name;
  guard->value = NULL;
  guard->was_set = value ? 1 : 0;
  if (value) {
    guard->value = dup_string(value);
    if (!guard->value) {
      printf("ERROR: Memory allocation failure\n");
      return -1;
    }
  }

  return 0;
}

static void free_env_guard(env_guard *guard)
{
  free(guard->value);
  guard->value = NULL;
}

static int set_env_value(const char *name, const char *value)
{
#ifdef _WIN32
  return _putenv_s(name, value);
#else
  return setenv(name, value, 1);
#endif
}

static int unset_env_value(const char *name)
{
#ifdef _WIN32
  return _putenv_s(name, "");
#else
  return unsetenv(name);
#endif
}

static int restore_env(const env_guard *guard)
{
  if (!guard->name)
    return 0;
  if (guard->was_set)
    return set_env_value(guard->name, guard->value);

  return unset_env_value(guard->name);
}

static void init_image(unsigned char *src_buf, int width, int height)
{
  int x, y;

  for (y = 0; y < height; y++) {
    for (x = 0; x < width; x++) {
      int index = (y * width + x) * 3;

      src_buf[index + 0] = (y < height / 2) ? 0 : 255;
      src_buf[index + 1] = (x < width / 2) ? 64 : 192;
      src_buf[index + 2] = ((x / 8 + y / 8) & 1) ? 32 : 224;
    }
  }
}

static int try_compress_test_jpeg(unsigned char **jpeg_buf,
                                  unsigned long *jpeg_size,
                                  char *error_message)
{
  tjhandle handle = NULL;
  unsigned char src_buf[32 * 32 * 3];
  int retval = 0;

  init_image(src_buf, 32, 32);
  *jpeg_buf = NULL;
  *jpeg_size = 0;
  if (error_message)
    error_message[0] = '\0';

  handle = tjInitCompress();
  if (!handle) {
    if (error_message) {
      strncpy(error_message, tjGetErrorStr2(NULL), JMSG_LENGTH_MAX);
      error_message[JMSG_LENGTH_MAX - 1] = '\0';
    }
    return -1;
  }

  if (tjCompress2(handle, src_buf, 32, 0, 32, TJPF_RGB, jpeg_buf, jpeg_size,
                  TJSAMP_444, 75, 0) == -1) {
    if (error_message) {
      strncpy(error_message, tjGetErrorStr2(handle), JMSG_LENGTH_MAX);
      error_message[JMSG_LENGTH_MAX - 1] = '\0';
    }
    retval = -1;
  }

  tjDestroy(handle);
  return retval;
}

static int compress_test_jpeg(unsigned char **jpeg_buf, unsigned long *jpeg_size)
{
  char error_message[JMSG_LENGTH_MAX];

  if (try_compress_test_jpeg(jpeg_buf, jpeg_size, error_message) == -1) {
    printf("TurboJPEG ERROR: %s\n", error_message);
    return -1;
  }

  return 0;
}

static void init_memory_src(j_decompress_ptr cinfo)
{
  (void)cinfo;
}

static boolean fill_memory_src(j_decompress_ptr cinfo)
{
  static const JOCTET buffer[2] = { 0xFF, JPEG_EOI };

  cinfo->src->next_input_byte = buffer;
  cinfo->src->bytes_in_buffer = sizeof(buffer);
  return TRUE;
}

static void skip_memory_src(j_decompress_ptr cinfo, long num_bytes)
{
  if (num_bytes <= 0)
    return;

  while (num_bytes > (long)cinfo->src->bytes_in_buffer) {
    num_bytes -= (long)cinfo->src->bytes_in_buffer;
    fill_memory_src(cinfo);
  }

  cinfo->src->next_input_byte += num_bytes;
  cinfo->src->bytes_in_buffer -= (size_t)num_bytes;
}

static void term_memory_src(j_decompress_ptr cinfo)
{
  (void)cinfo;
}

/* Avoid depending on the optional jpeg_mem_src() helper. */
static void set_memory_src(j_decompress_ptr cinfo, const unsigned char *jpeg_buf,
                           unsigned long jpeg_size)
{
  memory_src_mgr *src;

  if (!cinfo->src) {
    cinfo->src = (struct jpeg_source_mgr *)
      (*cinfo->mem->alloc_small)((j_common_ptr)cinfo, JPOOL_PERMANENT,
                                 sizeof(memory_src_mgr));
  }

  src = (memory_src_mgr *)cinfo->src;
  src->pub.init_source = init_memory_src;
  src->pub.fill_input_buffer = fill_memory_src;
  src->pub.skip_input_data = skip_memory_src;
  src->pub.resync_to_restart = jpeg_resync_to_restart;
  src->pub.term_source = term_memory_src;
  src->pub.bytes_in_buffer = (size_t)jpeg_size;
  src->pub.next_input_byte = (const JOCTET *)jpeg_buf;
}

static int read_jpeg_header(const unsigned char *jpeg_buf, unsigned long jpeg_size,
                            jpeg_header_info *info)
{
  struct jpeg_decompress_struct cinfo;
  error_mgr jerr;

  memset(&cinfo, 0, sizeof(cinfo));
  memset(&jerr, 0, sizeof(jerr));
  info->progressive_mode = 0;
  info->arith_code = 0;
  info->restart_interval = 0;

  cinfo.err = jpeg_std_error(&jerr.pub);
  jerr.pub.error_exit = my_error_exit;

  if (setjmp(jerr.jb)) {
    printf("libjpeg ERROR: %s\n", jerr.message);
    jpeg_destroy_decompress(&cinfo);
    return -1;
  }

  jpeg_create_decompress(&cinfo);
  set_memory_src(&cinfo, jpeg_buf, jpeg_size);
  jpeg_read_header(&cinfo, TRUE);

  info->progressive_mode = cinfo.progressive_mode ? 1 : 0;
  info->arith_code = cinfo.arith_code ? 1 : 0;
  info->restart_interval = cinfo.restart_interval;

  jpeg_destroy_decompress(&cinfo);
  return 0;
}

static int with_env(const char *name, const char *value, env_guard *guard)
{
  if (save_env(guard, name) == -1)
    return -1;
  if (set_env_value(name, value) == -1) {
    printf("ERROR: Could not set %s\n", name);
    free_env_guard(guard);
    return -1;
  }

  return 0;
}

static int clear_env(const char *name, env_guard *guard)
{
  if (save_env(guard, name) == -1)
    return -1;
  if (unset_env_value(name) == -1) {
    printf("ERROR: Could not clear %s\n", name);
    free_env_guard(guard);
    return -1;
  }

  return 0;
}

static int verify_header(const char *label, int expected_progressive,
                         int expected_arith, unsigned int expected_restart,
                         const char *unsupported_error)
{
  unsigned char *jpeg_buf = NULL;
  unsigned long jpeg_size = 0;
  jpeg_header_info info;
  char error_message[JMSG_LENGTH_MAX];
  int retval = -1;

  printf("%s...\n", label);

  if (try_compress_test_jpeg(&jpeg_buf, &jpeg_size, error_message) == -1) {
    if (unsupported_error && !strcmp(error_message, unsupported_error)) {
      printf("%s\n", error_message);
      printf("SUCCESS!\n\n");
      retval = 0;
      goto bailout;
    }
    printf("TurboJPEG ERROR: %s\n", error_message);
    goto bailout;
  }
  if (read_jpeg_header(jpeg_buf, jpeg_size, &info) == -1)
    goto bailout;

  if (info.progressive_mode != expected_progressive) {
    printf("ERROR: progressive_mode is %d, should be %d\n",
           info.progressive_mode, expected_progressive);
    goto bailout;
  }
  if (info.arith_code != expected_arith) {
    printf("ERROR: arith_code is %d, should be %d\n",
           info.arith_code, expected_arith);
    goto bailout;
  }
  if (info.restart_interval != expected_restart) {
    printf("ERROR: restart_interval is %u, should be %u\n",
           info.restart_interval, expected_restart);
    goto bailout;
  }

  printf("SUCCESS!\n\n");
  retval = 0;

bailout:
  tjFree(jpeg_buf);
  return retval;
}

static int verify_default_settings(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  int retval = -1;

  if (clear_env("TJ_OPTIMIZE", &optimize) == -1 ||
      clear_env("TJ_ARITHMETIC", &arithmetic) == -1 ||
      clear_env("TJ_RESTART", &restart) == -1 ||
      clear_env("TJ_PROGRESSIVE", &progressive) == -1)
    goto bailout;

  retval = verify_header("Default TurboJPEG environment", 0, 0, 0, NULL);

bailout:
  restore_env(&progressive);
  restore_env(&restart);
  restore_env(&arithmetic);
  restore_env(&optimize);
  free_env_guard(&progressive);
  free_env_guard(&restart);
  free_env_guard(&arithmetic);
  free_env_guard(&optimize);
  return retval;
}

static int verify_optimize_setting(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  unsigned char *default_buf = NULL, *optimized_buf = NULL;
  unsigned long default_size = 0, optimized_size = 0;
  int retval = -1;

  printf("TJ_OPTIMIZE...\n");

  if (clear_env("TJ_ARITHMETIC", &arithmetic) == -1 ||
      clear_env("TJ_RESTART", &restart) == -1 ||
      clear_env("TJ_PROGRESSIVE", &progressive) == -1)
    goto bailout;

  if (save_env(&optimize, "TJ_OPTIMIZE") == -1)
    goto bailout;
  if (unset_env_value("TJ_OPTIMIZE") == -1) {
    printf("ERROR: Could not clear TJ_OPTIMIZE\n");
    goto bailout;
  }
  if (compress_test_jpeg(&default_buf, &default_size) == -1)
    goto bailout;

  if (set_env_value("TJ_OPTIMIZE", "1") == -1) {
    printf("ERROR: Could not set TJ_OPTIMIZE\n");
    goto bailout;
  }
  if (compress_test_jpeg(&optimized_buf, &optimized_size) == -1)
    goto bailout;

  if (optimized_size >= default_size) {
    printf("ERROR: Optimized JPEG is %lu bytes, default JPEG is %lu bytes\n",
           optimized_size, default_size);
    goto bailout;
  }

  printf("SUCCESS!\n\n");
  retval = 0;

bailout:
  restore_env(&progressive);
  restore_env(&restart);
  restore_env(&arithmetic);
  restore_env(&optimize);
  free_env_guard(&progressive);
  free_env_guard(&restart);
  free_env_guard(&arithmetic);
  free_env_guard(&optimize);
  tjFree(optimized_buf);
  tjFree(default_buf);
  return retval;
}

static int verify_restart_interval(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  int retval = -1;

  if (clear_env("TJ_OPTIMIZE", &optimize) == -1 ||
      clear_env("TJ_ARITHMETIC", &arithmetic) == -1 ||
      clear_env("TJ_PROGRESSIVE", &progressive) == -1 ||
      with_env("TJ_RESTART", "8B", &restart) == -1)
    goto bailout;

  retval = verify_header("TJ_RESTART = 8B", 0, 0, 8, NULL);

bailout:
  restore_env(&restart);
  restore_env(&progressive);
  restore_env(&arithmetic);
  restore_env(&optimize);
  free_env_guard(&restart);
  free_env_guard(&progressive);
  free_env_guard(&arithmetic);
  free_env_guard(&optimize);
  return retval;
}

static int verify_restart_rows(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  int retval = -1;

  if (clear_env("TJ_OPTIMIZE", &optimize) == -1 ||
      clear_env("TJ_ARITHMETIC", &arithmetic) == -1 ||
      clear_env("TJ_PROGRESSIVE", &progressive) == -1 ||
      with_env("TJ_RESTART", "1", &restart) == -1)
    goto bailout;

  retval = verify_header("TJ_RESTART = 1 MCU row", 0, 0, 4, NULL);

bailout:
  restore_env(&restart);
  restore_env(&progressive);
  restore_env(&arithmetic);
  restore_env(&optimize);
  free_env_guard(&restart);
  free_env_guard(&progressive);
  free_env_guard(&arithmetic);
  free_env_guard(&optimize);
  return retval;
}

static int verify_progressive_setting(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  int retval = -1;

  if (clear_env("TJ_OPTIMIZE", &optimize) == -1 ||
      clear_env("TJ_ARITHMETIC", &arithmetic) == -1 ||
      clear_env("TJ_RESTART", &restart) == -1 ||
      with_env("TJ_PROGRESSIVE", "1", &progressive) == -1)
    goto bailout;

  retval = verify_header("TJ_PROGRESSIVE = 1", 1, 0, 0,
                         "Requested feature was omitted at compile time");

bailout:
  restore_env(&progressive);
  restore_env(&restart);
  restore_env(&arithmetic);
  restore_env(&optimize);
  free_env_guard(&progressive);
  free_env_guard(&restart);
  free_env_guard(&arithmetic);
  free_env_guard(&optimize);
  return retval;
}

static int verify_arithmetic_setting(void)
{
  env_guard optimize = { 0 }, arithmetic = { 0 }, restart = { 0 },
    progressive = { 0 };
  int retval = -1;

  if (clear_env("TJ_OPTIMIZE", &optimize) == -1 ||
      clear_env("TJ_PROGRESSIVE", &progressive) == -1 ||
      clear_env("TJ_RESTART", &restart) == -1 ||
      with_env("TJ_ARITHMETIC", "1", &arithmetic) == -1)
    goto bailout;

  retval = verify_header("TJ_ARITHMETIC = 1", 0, 1, 0,
                         "Sorry, arithmetic coding is not implemented");

bailout:
  restore_env(&arithmetic);
  restore_env(&restart);
  restore_env(&progressive);
  restore_env(&optimize);
  free_env_guard(&arithmetic);
  free_env_guard(&restart);
  free_env_guard(&progressive);
  free_env_guard(&optimize);
  return retval;
}

int main(void)
{
  if (verify_default_settings() == -1 ||
      verify_optimize_setting() == -1 ||
      verify_restart_interval() == -1 ||
      verify_restart_rows() == -1 ||
      verify_progressive_setting() == -1 ||
      verify_arithmetic_setting() == -1)
    return -1;

  return 0;
}
