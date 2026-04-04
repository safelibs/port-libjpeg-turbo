/*
 * Copyright (C)2011-2013, 2016, 2020 D. R. Commander.  All Rights Reserved.
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

package org.libjpegturbo.turbojpeg;

final class TJLoader {
  private static void loadUnixLibrary(String libdir) {
    try {
      System.load(libdir + "/libturbojpeg.so.0");
    } catch (java.lang.UnsatisfiedLinkError e) {
      System.load(libdir + "/libturbojpeg.so");
    }
  }

  static void load() {
    try {
      System.loadLibrary("turbojpeg");
    } catch (java.lang.UnsatisfiedLinkError e) {
      String os = System.getProperty("os.name").toLowerCase();
      if (os.indexOf("mac") >= 0) {
        try {
          System.load("/usr/lib/x86_64-linux-gnu/libturbojpeg.dylib");
        } catch (java.lang.UnsatisfiedLinkError e2) {
          System.load("/usr/lib/libturbojpeg.dylib");
        }
      } else {
        try {
          loadUnixLibrary("/usr/lib/x86_64-linux-gnu");
        } catch (java.lang.UnsatisfiedLinkError e3) {
          String libdir = "/usr/lib/x86_64-linux-gnu";
          if (libdir.equals("/usr/lib64")) {
            loadUnixLibrary("/usr/lib32");
          } else if (libdir.equals("/usr/lib32")) {
            loadUnixLibrary("/usr/lib64");
          } else {
            throw e3;
          }
        }
      }
    }
  }
}
