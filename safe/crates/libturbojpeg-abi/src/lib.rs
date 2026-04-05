#![allow(clippy::all)]

pub mod generated;

pub const SONAME: &str = "libturbojpeg.so.0";
pub const LINK_NAME: &str = "turbojpeg";
pub const JNI_SOURCE_FILE: &str = "turbojpeg-jni.c";
pub const NON_JNI_SOURCE_FILES: &[&str] = &[
    jpeg_core::ported::turbojpeg::turbojpeg::SOURCE_FILE,
    jpeg_core::ported::turbojpeg::jdatasrc_tj::SOURCE_FILE,
    jpeg_core::ported::turbojpeg::jdatadst_tj::SOURCE_FILE,
    jpeg_core::ported::turbojpeg::tjutil::SOURCE_FILE,
];
pub const HELPER_IMAGE_IO_SOURCE_FILES: &[&str] = &["rdbmp.c", "rdppm.c", "wrbmp.c", "wrppm.c"];
pub const TURBOJPEG_TOOL_SOURCE_FILES: &[&str] = &["tjbench.c", "tjexample.c"];

pub const VERSIONED_EXPORTS: &[(&str, &str)] = &[
    ("TJBUFSIZE", "TURBOJPEG_1.0"),
    ("tjCompress", "TURBOJPEG_1.0"),
    ("tjDecompress", "TURBOJPEG_1.0"),
    ("tjDecompressHeader", "TURBOJPEG_1.0"),
    ("tjDestroy", "TURBOJPEG_1.0"),
    ("tjGetErrorStr", "TURBOJPEG_1.0"),
    ("tjInitCompress", "TURBOJPEG_1.0"),
    ("tjInitDecompress", "TURBOJPEG_1.0"),
    ("TJBUFSIZEYUV", "TURBOJPEG_1.1"),
    ("tjDecompressHeader2", "TURBOJPEG_1.1"),
    ("tjDecompressToYUV", "TURBOJPEG_1.1"),
    ("tjEncodeYUV", "TURBOJPEG_1.1"),
    ("tjAlloc", "TURBOJPEG_1.2"),
    ("tjBufSize", "TURBOJPEG_1.2"),
    ("tjBufSizeYUV", "TURBOJPEG_1.2"),
    ("tjCompress2", "TURBOJPEG_1.2"),
    ("tjDecompress2", "TURBOJPEG_1.2"),
    ("tjEncodeYUV2", "TURBOJPEG_1.2"),
    ("tjFree", "TURBOJPEG_1.2"),
    ("tjGetScalingFactors", "TURBOJPEG_1.2"),
    ("tjInitTransform", "TURBOJPEG_1.2"),
    ("tjTransform", "TURBOJPEG_1.2"),
    ("tjBufSizeYUV2", "TURBOJPEG_1.4"),
    ("tjCompressFromYUV", "TURBOJPEG_1.4"),
    ("tjCompressFromYUVPlanes", "TURBOJPEG_1.4"),
    ("tjDecodeYUV", "TURBOJPEG_1.4"),
    ("tjDecodeYUVPlanes", "TURBOJPEG_1.4"),
    ("tjDecompressHeader3", "TURBOJPEG_1.4"),
    ("tjDecompressToYUV2", "TURBOJPEG_1.4"),
    ("tjDecompressToYUVPlanes", "TURBOJPEG_1.4"),
    ("tjEncodeYUV3", "TURBOJPEG_1.4"),
    ("tjEncodeYUVPlanes", "TURBOJPEG_1.4"),
    ("tjPlaneHeight", "TURBOJPEG_1.4"),
    ("tjPlaneSizeYUV", "TURBOJPEG_1.4"),
    ("tjPlaneWidth", "TURBOJPEG_1.4"),
    ("tjGetErrorCode", "TURBOJPEG_2.0"),
    ("tjGetErrorStr2", "TURBOJPEG_2.0"),
    ("tjLoadImage", "TURBOJPEG_2.0"),
    ("tjSaveImage", "TURBOJPEG_2.0"),
];

pub const EXPECTED_NON_JNI_SYMBOLS: &[&str] = &[
    "TJBUFSIZE",
    "TJBUFSIZEYUV",
    "tjAlloc",
    "tjBufSize",
    "tjBufSizeYUV",
    "tjBufSizeYUV2",
    "tjCompress",
    "tjCompress2",
    "tjCompressFromYUV",
    "tjCompressFromYUVPlanes",
    "tjDecodeYUV",
    "tjDecodeYUVPlanes",
    "tjDecompress",
    "tjDecompress2",
    "tjDecompressHeader",
    "tjDecompressHeader2",
    "tjDecompressHeader3",
    "tjDecompressToYUV",
    "tjDecompressToYUV2",
    "tjDecompressToYUVPlanes",
    "tjDestroy",
    "tjEncodeYUV",
    "tjEncodeYUV2",
    "tjEncodeYUV3",
    "tjEncodeYUVPlanes",
    "tjFree",
    "tjGetErrorCode",
    "tjGetErrorStr",
    "tjGetErrorStr2",
    "tjGetScalingFactors",
    "tjInitCompress",
    "tjInitDecompress",
    "tjInitTransform",
    "tjLoadImage",
    "tjPlaneHeight",
    "tjPlaneSizeYUV",
    "tjPlaneWidth",
    "tjSaveImage",
    "tjTransform",
];

pub const EXPECTED_JNI_SYMBOLS: &[&str] = &[
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compressFromYUV___3_3B_3II_3III_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIII_3BIII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_destroy",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIIIII_3_3B_3I_3III",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIII_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIIIII_3_3B_3I_3III",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIII_3BII",
    "Java_org_libjpegturbo_turbojpeg_TJCompressor_init",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3BIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3IIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressHeader",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3BI",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3_3B_3II_3III",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIIIII",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_destroy",
    "Java_org_libjpegturbo_turbojpeg_TJDecompressor_init",
    "Java_org_libjpegturbo_turbojpeg_TJTransformer_init",
    "Java_org_libjpegturbo_turbojpeg_TJTransformer_transform",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSize",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__III",
    "Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__IIII",
    "Java_org_libjpegturbo_turbojpeg_TJ_getScalingFactors",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeHeight__III",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeSizeYUV__IIIII",
    "Java_org_libjpegturbo_turbojpeg_TJ_planeWidth__III",
];

pub fn expected_packaged_symbol_names() -> impl Iterator<Item = &'static str> {
    EXPECTED_NON_JNI_SYMBOLS
        .iter()
        .copied()
        .chain(EXPECTED_JNI_SYMBOLS.iter().copied())
}

pub fn version_for_export(symbol: &str) -> Option<&'static str> {
    VERSIONED_EXPORTS
        .iter()
        .find_map(|(name, version)| (*name == symbol).then_some(*version))
}

pub mod non_jni {
    pub use crate::generated::turbojpeg::{
        tjAlloc, tjCompress2, tjDecompress2, tjDecompressHeader3, tjDestroy, tjFree,
        tjGetErrorCode, tjGetErrorStr, tjGetErrorStr2, tjInitCompress, tjInitDecompress,
        tjInitTransform, tjLoadImage, tjSaveImage, tjTransform, tjhandle, tjregion,
        tjscalingfactor, tjtransform, TJCS_YCbCr, TJCS_CMYK, TJCS_GRAY, TJCS_RGB, TJCS_YCCK,
        TJERR_FATAL, TJERR_WARNING, TJFLAG_ACCURATEDCT, TJFLAG_BOTTOMUP, TJFLAG_FASTDCT,
        TJFLAG_FASTUPSAMPLE, TJPF_BGRX, TJPF_GRAY, TJPF_UNKNOWN, TJSAMP_420, TJSAMP_422,
        TJSAMP_444, TJSAMP_GRAY, TJXOPT_COPYNONE, TJXOPT_CROP, TJXOPT_GRAY, TJXOPT_NOOUTPUT,
        TJXOPT_PERFECT, TJXOPT_PROGRESSIVE, TJXOPT_TRIM, TJXOP_HFLIP, TJXOP_NONE, TJXOP_ROT180,
        TJXOP_ROT270, TJXOP_ROT90, TJXOP_TRANSPOSE, TJXOP_TRANSVERSE, TJXOP_VFLIP,
    };
}
