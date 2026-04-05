#[repr(C)]
pub struct _jobject {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _jfieldID {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _jmethodID {
    _unused: [u8; 0],
}
extern "C" {
    fn tjInitCompress() -> tjhandle;
    fn tjCompress2(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        jpegBuf: *mut *mut ::core::ffi::c_uchar,
        jpegSize: *mut ::core::ffi::c_ulong,
        jpegSubsamp: ::core::ffi::c_int,
        jpegQual: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjCompressFromYUVPlanes(
        handle: tjhandle,
        srcPlanes: *mut *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        strides: *const ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        jpegBuf: *mut *mut ::core::ffi::c_uchar,
        jpegSize: *mut ::core::ffi::c_ulong,
        jpegQual: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjBufSize(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        jpegSubsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjBufSizeYUV2(
        width: ::core::ffi::c_int,
        align: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjPlaneSizeYUV(
        componentID: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        stride: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjPlaneWidth(
        componentID: ::core::ffi::c_int,
        width: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjPlaneHeight(
        componentID: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjEncodeYUVPlanes(
        handle: tjhandle,
        srcBuf: *const ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        dstPlanes: *mut *mut ::core::ffi::c_uchar,
        strides: *mut ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjInitDecompress() -> tjhandle;
    fn tjDecompressHeader3(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
        jpegSubsamp: *mut ::core::ffi::c_int,
        jpegColorspace: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjGetScalingFactors(numScalingFactors: *mut ::core::ffi::c_int) -> *mut tjscalingfactor;
    fn tjDecompress2(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        dstBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDecompressToYUVPlanes(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        dstPlanes: *mut *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        strides: *mut ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDecodeYUVPlanes(
        handle: tjhandle,
        srcPlanes: *mut *const ::core::ffi::c_uchar,
        strides: *const ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
        dstBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjInitTransform() -> tjhandle;
    fn tjTransform(
        handle: tjhandle,
        jpegBuf: *const ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        n: ::core::ffi::c_int,
        dstBufs: *mut *mut ::core::ffi::c_uchar,
        dstSizes: *mut ::core::ffi::c_ulong,
        transforms: *mut tjtransform,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjDestroy(handle: tjhandle) -> ::core::ffi::c_int;
    fn tjGetErrorStr2(handle: tjhandle) -> *mut ::core::ffi::c_char;
    fn tjGetErrorCode(handle: tjhandle) -> ::core::ffi::c_int;
    fn tjGetErrorStr() -> *mut ::core::ffi::c_char;
    fn tjDecompressToYUV(
        handle: tjhandle,
        jpegBuf: *mut ::core::ffi::c_uchar,
        jpegSize: ::core::ffi::c_ulong,
        dstBuf: *mut ::core::ffi::c_uchar,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn tjBufSizeYUV(
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        subsamp: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn tjEncodeYUV2(
        handle: tjhandle,
        srcBuf: *mut ::core::ffi::c_uchar,
        width: ::core::ffi::c_int,
        pitch: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
        pixelFormat: ::core::ffi::c_int,
        dstBuf: *mut ::core::ffi::c_uchar,
        subsamp: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __errno_location() -> *mut ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjscalingfactor {
    pub num: ::core::ffi::c_int,
    pub denom: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjregion {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tjtransform {
    pub r: tjregion,
    pub op: ::core::ffi::c_int,
    pub options: ::core::ffi::c_int,
    pub data: *mut ::core::ffi::c_void,
    pub customFilter: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_short,
            tjregion,
            tjregion,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut tjtransform,
        ) -> ::core::ffi::c_int,
    >,
}
pub type tjhandle = *mut ::core::ffi::c_void;
pub type size_t = usize;
pub type jint = ::core::ffi::c_int;
pub type jlong = ::core::ffi::c_long;
pub type jbyte = ::core::ffi::c_schar;
pub type jboolean = ::core::ffi::c_uchar;
pub type jchar = ::core::ffi::c_ushort;
pub type jshort = ::core::ffi::c_short;
pub type jfloat = ::core::ffi::c_float;
pub type jdouble = ::core::ffi::c_double;
pub type jsize = jint;
pub type jobject = *mut _jobject;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jarray;
pub type jbyteArray = jarray;
pub type jcharArray = jarray;
pub type jshortArray = jarray;
pub type jintArray = jarray;
pub type jlongArray = jarray;
pub type jfloatArray = jarray;
pub type jdoubleArray = jarray;
pub type jobjectArray = jarray;
pub type jweak = jobject;
#[derive(Copy, Clone)]
#[repr(C)]
pub union jvalue {
    pub z: jboolean,
    pub b: jbyte,
    pub c: jchar,
    pub s: jshort,
    pub i: jint,
    pub j: jlong,
    pub f: jfloat,
    pub d: jdouble,
    pub l: jobject,
}
pub type jfieldID = *mut _jfieldID;
pub type jmethodID = *mut _jmethodID;
pub type _jobjectType = ::core::ffi::c_uint;
pub const JNIWeakGlobalRefType: _jobjectType = 3;
pub const JNIGlobalRefType: _jobjectType = 2;
pub const JNILocalRefType: _jobjectType = 1;
pub const JNIInvalidRefType: _jobjectType = 0;
pub type jobjectRefType = _jobjectType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JNINativeMethod {
    pub name: *mut ::core::ffi::c_char,
    pub signature: *mut ::core::ffi::c_char,
    pub fnPtr: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JNINativeInterface_ {
    pub reserved0: *mut ::core::ffi::c_void,
    pub reserved1: *mut ::core::ffi::c_void,
    pub reserved2: *mut ::core::ffi::c_void,
    pub reserved3: *mut ::core::ffi::c_void,
    pub GetVersion: Option<unsafe extern "C" fn(*mut JNIEnv) -> jint>,
    pub DefineClass: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            *const ::core::ffi::c_char,
            jobject,
            *const jbyte,
            jsize,
        ) -> jclass,
    >,
    pub FindClass: Option<unsafe extern "C" fn(*mut JNIEnv, *const ::core::ffi::c_char) -> jclass>,
    pub FromReflectedMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jmethodID>,
    pub FromReflectedField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jfieldID>,
    pub ToReflectedMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, jboolean) -> jobject>,
    pub GetSuperclass: Option<unsafe extern "C" fn(*mut JNIEnv, jclass) -> jclass>,
    pub IsAssignableFrom: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jclass) -> jboolean>,
    pub ToReflectedField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jboolean) -> jobject>,
    pub Throw: Option<unsafe extern "C" fn(*mut JNIEnv, jthrowable) -> jint>,
    pub ThrowNew:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, *const ::core::ffi::c_char) -> jint>,
    pub ExceptionOccurred: Option<unsafe extern "C" fn(*mut JNIEnv) -> jthrowable>,
    pub ExceptionDescribe: Option<unsafe extern "C" fn(*mut JNIEnv) -> ()>,
    pub ExceptionClear: Option<unsafe extern "C" fn(*mut JNIEnv) -> ()>,
    pub FatalError: Option<unsafe extern "C" fn(*mut JNIEnv, *const ::core::ffi::c_char) -> ()>,
    pub PushLocalFrame: Option<unsafe extern "C" fn(*mut JNIEnv, jint) -> jint>,
    pub PopLocalFrame: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject>,
    pub NewGlobalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject>,
    pub DeleteGlobalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> ()>,
    pub DeleteLocalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> ()>,
    pub IsSameObject: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jobject) -> jboolean>,
    pub NewLocalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobject>,
    pub EnsureLocalCapacity: Option<unsafe extern "C" fn(*mut JNIEnv, jint) -> jint>,
    pub AllocObject: Option<unsafe extern "C" fn(*mut JNIEnv, jclass) -> jobject>,
    pub NewObject: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jobject>,
    pub NewObjectV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jobject,
    >,
    pub NewObjectA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject>,
    pub GetObjectClass: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jclass>,
    pub IsInstanceOf: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass) -> jboolean>,
    pub GetMethodID: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jclass,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> jmethodID,
    >,
    pub CallObjectMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jobject>,
    pub CallObjectMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jobject,
    >,
    pub CallObjectMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jobject>,
    pub CallBooleanMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jboolean>,
    pub CallBooleanMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jboolean,
    >,
    pub CallBooleanMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jboolean>,
    pub CallByteMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jbyte>,
    pub CallByteMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jbyte,
    >,
    pub CallByteMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jbyte>,
    pub CallCharMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jchar>,
    pub CallCharMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jchar,
    >,
    pub CallCharMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jchar>,
    pub CallShortMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jshort>,
    pub CallShortMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jshort,
    >,
    pub CallShortMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jshort>,
    pub CallIntMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jint>,
    pub CallIntMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jint,
    >,
    pub CallIntMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jint>,
    pub CallLongMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jlong>,
    pub CallLongMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jlong,
    >,
    pub CallLongMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jlong>,
    pub CallFloatMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jfloat>,
    pub CallFloatMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jfloat,
    >,
    pub CallFloatMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jfloat>,
    pub CallDoubleMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> jdouble>,
    pub CallDoubleMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> jdouble,
    >,
    pub CallDoubleMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> jdouble>,
    pub CallVoidMethod: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, ...) -> ()>,
    pub CallVoidMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *mut ::core::ffi::c_void) -> (),
    >,
    pub CallVoidMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jmethodID, *const jvalue) -> ()>,
    pub CallNonvirtualObjectMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jobject>,
    pub CallNonvirtualObjectMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jobject,
    >,
    pub CallNonvirtualObjectMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jobject,
    >,
    pub CallNonvirtualBooleanMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jboolean>,
    pub CallNonvirtualBooleanMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jboolean,
    >,
    pub CallNonvirtualBooleanMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jboolean,
    >,
    pub CallNonvirtualByteMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jbyte>,
    pub CallNonvirtualByteMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jbyte,
    >,
    pub CallNonvirtualByteMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jbyte,
    >,
    pub CallNonvirtualCharMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jchar>,
    pub CallNonvirtualCharMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jchar,
    >,
    pub CallNonvirtualCharMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jchar,
    >,
    pub CallNonvirtualShortMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jshort>,
    pub CallNonvirtualShortMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jshort,
    >,
    pub CallNonvirtualShortMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jshort,
    >,
    pub CallNonvirtualIntMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jint>,
    pub CallNonvirtualIntMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jint,
    >,
    pub CallNonvirtualIntMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jint,
    >,
    pub CallNonvirtualLongMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jlong>,
    pub CallNonvirtualLongMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jlong,
    >,
    pub CallNonvirtualLongMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jlong,
    >,
    pub CallNonvirtualFloatMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jfloat>,
    pub CallNonvirtualFloatMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jfloat,
    >,
    pub CallNonvirtualFloatMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jfloat,
    >,
    pub CallNonvirtualDoubleMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> jdouble>,
    pub CallNonvirtualDoubleMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> jdouble,
    >,
    pub CallNonvirtualDoubleMethodA: Option<
        unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> jdouble,
    >,
    pub CallNonvirtualVoidMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, ...) -> ()>,
    pub CallNonvirtualVoidMethodV: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jobject,
            jclass,
            jmethodID,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
    pub CallNonvirtualVoidMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jclass, jmethodID, *const jvalue) -> ()>,
    pub GetFieldID: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jclass,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> jfieldID,
    >,
    pub GetObjectField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jobject>,
    pub GetBooleanField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jboolean>,
    pub GetByteField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jbyte>,
    pub GetCharField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jchar>,
    pub GetShortField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jshort>,
    pub GetIntField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jint>,
    pub GetLongField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jlong>,
    pub GetFloatField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jfloat>,
    pub GetDoubleField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID) -> jdouble>,
    pub SetObjectField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jobject) -> ()>,
    pub SetBooleanField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jboolean) -> ()>,
    pub SetByteField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jbyte) -> ()>,
    pub SetCharField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jchar) -> ()>,
    pub SetShortField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jshort) -> ()>,
    pub SetIntField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jint) -> ()>,
    pub SetLongField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jlong) -> ()>,
    pub SetFloatField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jfloat) -> ()>,
    pub SetDoubleField: Option<unsafe extern "C" fn(*mut JNIEnv, jobject, jfieldID, jdouble) -> ()>,
    pub GetStaticMethodID: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jclass,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> jmethodID,
    >,
    pub CallStaticObjectMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jobject>,
    pub CallStaticObjectMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jobject,
    >,
    pub CallStaticObjectMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jobject>,
    pub CallStaticBooleanMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jboolean>,
    pub CallStaticBooleanMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jboolean,
    >,
    pub CallStaticBooleanMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jboolean>,
    pub CallStaticByteMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jbyte>,
    pub CallStaticByteMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jbyte,
    >,
    pub CallStaticByteMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jbyte>,
    pub CallStaticCharMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jchar>,
    pub CallStaticCharMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jchar,
    >,
    pub CallStaticCharMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jchar>,
    pub CallStaticShortMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jshort>,
    pub CallStaticShortMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jshort,
    >,
    pub CallStaticShortMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jshort>,
    pub CallStaticIntMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jint>,
    pub CallStaticIntMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jint,
    >,
    pub CallStaticIntMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jint>,
    pub CallStaticLongMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jlong>,
    pub CallStaticLongMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jlong,
    >,
    pub CallStaticLongMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jlong>,
    pub CallStaticFloatMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jfloat>,
    pub CallStaticFloatMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jfloat,
    >,
    pub CallStaticFloatMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jfloat>,
    pub CallStaticDoubleMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> jdouble>,
    pub CallStaticDoubleMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> jdouble,
    >,
    pub CallStaticDoubleMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> jdouble>,
    pub CallStaticVoidMethod:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, ...) -> ()>,
    pub CallStaticVoidMethodV: Option<
        unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *mut ::core::ffi::c_void) -> (),
    >,
    pub CallStaticVoidMethodA:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jmethodID, *const jvalue) -> ()>,
    pub GetStaticFieldID: Option<
        unsafe extern "C" fn(
            *mut JNIEnv,
            jclass,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
        ) -> jfieldID,
    >,
    pub GetStaticObjectField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jobject>,
    pub GetStaticBooleanField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jboolean>,
    pub GetStaticByteField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jbyte>,
    pub GetStaticCharField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jchar>,
    pub GetStaticShortField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jshort>,
    pub GetStaticIntField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jint>,
    pub GetStaticLongField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jlong>,
    pub GetStaticFloatField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jfloat>,
    pub GetStaticDoubleField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID) -> jdouble>,
    pub SetStaticObjectField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jobject) -> ()>,
    pub SetStaticBooleanField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jboolean) -> ()>,
    pub SetStaticByteField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jbyte) -> ()>,
    pub SetStaticCharField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jchar) -> ()>,
    pub SetStaticShortField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jshort) -> ()>,
    pub SetStaticIntField: Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jint) -> ()>,
    pub SetStaticLongField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jlong) -> ()>,
    pub SetStaticFloatField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jfloat) -> ()>,
    pub SetStaticDoubleField:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, jfieldID, jdouble) -> ()>,
    pub NewString: Option<unsafe extern "C" fn(*mut JNIEnv, *const jchar, jsize) -> jstring>,
    pub GetStringLength: Option<unsafe extern "C" fn(*mut JNIEnv, jstring) -> jsize>,
    pub GetStringChars:
        Option<unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const jchar>,
    pub ReleaseStringChars: Option<unsafe extern "C" fn(*mut JNIEnv, jstring, *const jchar) -> ()>,
    pub NewStringUTF:
        Option<unsafe extern "C" fn(*mut JNIEnv, *const ::core::ffi::c_char) -> jstring>,
    pub GetStringUTFLength: Option<unsafe extern "C" fn(*mut JNIEnv, jstring) -> jsize>,
    pub GetStringUTFChars: Option<
        unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const ::core::ffi::c_char,
    >,
    pub ReleaseStringUTFChars:
        Option<unsafe extern "C" fn(*mut JNIEnv, jstring, *const ::core::ffi::c_char) -> ()>,
    pub GetArrayLength: Option<unsafe extern "C" fn(*mut JNIEnv, jarray) -> jsize>,
    pub NewObjectArray:
        Option<unsafe extern "C" fn(*mut JNIEnv, jsize, jclass, jobject) -> jobjectArray>,
    pub GetObjectArrayElement:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobjectArray, jsize) -> jobject>,
    pub SetObjectArrayElement:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobjectArray, jsize, jobject) -> ()>,
    pub NewBooleanArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jbooleanArray>,
    pub NewByteArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jbyteArray>,
    pub NewCharArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jcharArray>,
    pub NewShortArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jshortArray>,
    pub NewIntArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jintArray>,
    pub NewLongArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jlongArray>,
    pub NewFloatArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jfloatArray>,
    pub NewDoubleArray: Option<unsafe extern "C" fn(*mut JNIEnv, jsize) -> jdoubleArray>,
    pub GetBooleanArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, *mut jboolean) -> *mut jboolean>,
    pub GetByteArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbyteArray, *mut jboolean) -> *mut jbyte>,
    pub GetCharArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jcharArray, *mut jboolean) -> *mut jchar>,
    pub GetShortArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jshortArray, *mut jboolean) -> *mut jshort>,
    pub GetIntArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jintArray, *mut jboolean) -> *mut jint>,
    pub GetLongArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jlongArray, *mut jboolean) -> *mut jlong>,
    pub GetFloatArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jboolean) -> *mut jfloat>,
    pub GetDoubleArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, *mut jboolean) -> *mut jdouble>,
    pub ReleaseBooleanArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, *mut jboolean, jint) -> ()>,
    pub ReleaseByteArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbyteArray, *mut jbyte, jint) -> ()>,
    pub ReleaseCharArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jcharArray, *mut jchar, jint) -> ()>,
    pub ReleaseShortArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jshortArray, *mut jshort, jint) -> ()>,
    pub ReleaseIntArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jintArray, *mut jint, jint) -> ()>,
    pub ReleaseLongArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jlongArray, *mut jlong, jint) -> ()>,
    pub ReleaseFloatArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jfloatArray, *mut jfloat, jint) -> ()>,
    pub ReleaseDoubleArrayElements:
        Option<unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, *mut jdouble, jint) -> ()>,
    pub GetBooleanArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, jsize, jsize, *mut jboolean) -> ()>,
    pub GetByteArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbyteArray, jsize, jsize, *mut jbyte) -> ()>,
    pub GetCharArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jcharArray, jsize, jsize, *mut jchar) -> ()>,
    pub GetShortArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jshortArray, jsize, jsize, *mut jshort) -> ()>,
    pub GetIntArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jintArray, jsize, jsize, *mut jint) -> ()>,
    pub GetLongArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jlongArray, jsize, jsize, *mut jlong) -> ()>,
    pub GetFloatArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *mut jfloat) -> ()>,
    pub GetDoubleArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, jsize, jsize, *mut jdouble) -> ()>,
    pub SetBooleanArrayRegion: Option<
        unsafe extern "C" fn(*mut JNIEnv, jbooleanArray, jsize, jsize, *const jboolean) -> (),
    >,
    pub SetByteArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jbyteArray, jsize, jsize, *const jbyte) -> ()>,
    pub SetCharArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jcharArray, jsize, jsize, *const jchar) -> ()>,
    pub SetShortArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jshortArray, jsize, jsize, *const jshort) -> ()>,
    pub SetIntArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jintArray, jsize, jsize, *const jint) -> ()>,
    pub SetLongArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jlongArray, jsize, jsize, *const jlong) -> ()>,
    pub SetFloatArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jfloatArray, jsize, jsize, *const jfloat) -> ()>,
    pub SetDoubleArrayRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jdoubleArray, jsize, jsize, *const jdouble) -> ()>,
    pub RegisterNatives:
        Option<unsafe extern "C" fn(*mut JNIEnv, jclass, *const JNINativeMethod, jint) -> jint>,
    pub UnregisterNatives: Option<unsafe extern "C" fn(*mut JNIEnv, jclass) -> jint>,
    pub MonitorEnter: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jint>,
    pub MonitorExit: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jint>,
    pub GetJavaVM: Option<unsafe extern "C" fn(*mut JNIEnv, *mut *mut JavaVM) -> jint>,
    pub GetStringRegion:
        Option<unsafe extern "C" fn(*mut JNIEnv, jstring, jsize, jsize, *mut jchar) -> ()>,
    pub GetStringUTFRegion: Option<
        unsafe extern "C" fn(*mut JNIEnv, jstring, jsize, jsize, *mut ::core::ffi::c_char) -> (),
    >,
    pub GetPrimitiveArrayCritical: Option<
        unsafe extern "C" fn(*mut JNIEnv, jarray, *mut jboolean) -> *mut ::core::ffi::c_void,
    >,
    pub ReleasePrimitiveArrayCritical:
        Option<unsafe extern "C" fn(*mut JNIEnv, jarray, *mut ::core::ffi::c_void, jint) -> ()>,
    pub GetStringCritical:
        Option<unsafe extern "C" fn(*mut JNIEnv, jstring, *mut jboolean) -> *const jchar>,
    pub ReleaseStringCritical:
        Option<unsafe extern "C" fn(*mut JNIEnv, jstring, *const jchar) -> ()>,
    pub NewWeakGlobalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jweak>,
    pub DeleteWeakGlobalRef: Option<unsafe extern "C" fn(*mut JNIEnv, jweak) -> ()>,
    pub ExceptionCheck: Option<unsafe extern "C" fn(*mut JNIEnv) -> jboolean>,
    pub NewDirectByteBuffer:
        Option<unsafe extern "C" fn(*mut JNIEnv, *mut ::core::ffi::c_void, jlong) -> jobject>,
    pub GetDirectBufferAddress:
        Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> *mut ::core::ffi::c_void>,
    pub GetDirectBufferCapacity: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jlong>,
    pub GetObjectRefType: Option<unsafe extern "C" fn(*mut JNIEnv, jobject) -> jobjectRefType>,
    pub GetModule: Option<unsafe extern "C" fn(*mut JNIEnv, jclass) -> jobject>,
}
pub type JNIEnv = *const JNINativeInterface_;
pub type JavaVM = *const JNIInvokeInterface_;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JNIInvokeInterface_ {
    pub reserved0: *mut ::core::ffi::c_void,
    pub reserved1: *mut ::core::ffi::c_void,
    pub reserved2: *mut ::core::ffi::c_void,
    pub DestroyJavaVM: Option<unsafe extern "C" fn(*mut JavaVM) -> jint>,
    pub AttachCurrentThread: Option<
        unsafe extern "C" fn(
            *mut JavaVM,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> jint,
    >,
    pub DetachCurrentThread: Option<unsafe extern "C" fn(*mut JavaVM) -> jint>,
    pub GetEnv:
        Option<unsafe extern "C" fn(*mut JavaVM, *mut *mut ::core::ffi::c_void, jint) -> jint>,
    pub AttachCurrentThreadAsDaemon: Option<
        unsafe extern "C" fn(
            *mut JavaVM,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> jint,
    >,
}
pub type JNICustomFilterParams = _JNICustomFilterParams;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _JNICustomFilterParams {
    pub env: *mut JNIEnv,
    pub tobj: jobject,
    pub cfobj: jobject,
}
pub const TJ_NUMSAMP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TJ_NUMPF: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
static mut tjPixelSize: [::core::ffi::c_int; 12] = [
    3 as ::core::ffi::c_int,
    3 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    1 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
    4 as ::core::ffi::c_int,
];
pub const TJFLAG_NOREALLOC: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
#[inline(always)]
unsafe extern "C" fn PUTENV_S(
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if name.is_null() || value.is_null() {
        let ref mut fresh0 = *__errno_location();
        *fresh0 = EINVAL;
        return *fresh0;
    }
    setenv(name, value, 1 as ::core::ffi::c_int);
    return *__errno_location();
}
pub const org_libjpegturbo_turbojpeg_TJ_NUMSAMP: ::core::ffi::c_long = 6 as ::core::ffi::c_long;
pub const org_libjpegturbo_turbojpeg_TJ_SAMP_GRAY: ::core::ffi::c_long = 3 as ::core::ffi::c_long;
pub const org_libjpegturbo_turbojpeg_TJ_NUMPF: ::core::ffi::c_long = 12 as ::core::ffi::c_long;
unsafe extern "C" fn ProcessSystemProperties(mut env: *mut JNIEnv) -> ::core::ffi::c_int {
    let mut cls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut mid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
    let mut jName: jstring = ::core::ptr::null_mut::<_jobject>();
    let mut jValue: jstring = ::core::ptr::null_mut::<_jobject>();
    let mut value: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    cls = (**env).FindClass.expect("non-null function pointer")(
        env,
        b"java/lang/System\0" as *const u8 as *const ::core::ffi::c_char,
    );
    if !(cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        mid = (**env)
            .GetStaticMethodID
            .expect("non-null function pointer")(
            env,
            cls,
            b"getProperty\0" as *const u8 as *const ::core::ffi::c_char,
            b"(Ljava/lang/String;)Ljava/lang/String;\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(mid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            jName = (**env).NewStringUTF.expect("non-null function pointer")(
                env,
                b"turbojpeg.optimize\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !jName.is_null() {
                let mut exception: jboolean = 0;
                jValue = (**env)
                    .CallStaticObjectMethod
                    .expect("non-null function pointer")(
                    env, cls, mid, jName
                ) as jstring;
                exception = (**env).ExceptionCheck.expect("non-null function pointer")(env);
                if !jValue.is_null() && exception == 0 && {
                    value = (**env)
                        .GetStringUTFChars
                        .expect("non-null function pointer")(
                        env,
                        jValue,
                        ::core::ptr::null_mut::<jboolean>(),
                    );
                    !value.is_null()
                } {
                    PUTENV_S(
                        b"TJ_OPTIMIZE\0" as *const u8 as *const ::core::ffi::c_char,
                        value,
                    );
                    (**env)
                        .ReleaseStringUTFChars
                        .expect("non-null function pointer")(env, jValue, value);
                }
            }
            jName = (**env).NewStringUTF.expect("non-null function pointer")(
                env,
                b"turbojpeg.arithmetic\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !jName.is_null() {
                let mut exception_0: jboolean = 0;
                jValue = (**env)
                    .CallStaticObjectMethod
                    .expect("non-null function pointer")(
                    env, cls, mid, jName
                ) as jstring;
                exception_0 = (**env).ExceptionCheck.expect("non-null function pointer")(env);
                if !jValue.is_null() && exception_0 == 0 && {
                    value = (**env)
                        .GetStringUTFChars
                        .expect("non-null function pointer")(
                        env,
                        jValue,
                        ::core::ptr::null_mut::<jboolean>(),
                    );
                    !value.is_null()
                } {
                    PUTENV_S(
                        b"TJ_ARITHMETIC\0" as *const u8 as *const ::core::ffi::c_char,
                        value,
                    );
                    (**env)
                        .ReleaseStringUTFChars
                        .expect("non-null function pointer")(env, jValue, value);
                }
            }
            jName = (**env).NewStringUTF.expect("non-null function pointer")(
                env,
                b"turbojpeg.restart\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !jName.is_null() {
                let mut exception_1: jboolean = 0;
                jValue = (**env)
                    .CallStaticObjectMethod
                    .expect("non-null function pointer")(
                    env, cls, mid, jName
                ) as jstring;
                exception_1 = (**env).ExceptionCheck.expect("non-null function pointer")(env);
                if !jValue.is_null() && exception_1 == 0 && {
                    value = (**env)
                        .GetStringUTFChars
                        .expect("non-null function pointer")(
                        env,
                        jValue,
                        ::core::ptr::null_mut::<jboolean>(),
                    );
                    !value.is_null()
                } {
                    PUTENV_S(
                        b"TJ_RESTART\0" as *const u8 as *const ::core::ffi::c_char,
                        value,
                    );
                    (**env)
                        .ReleaseStringUTFChars
                        .expect("non-null function pointer")(env, jValue, value);
                }
            }
            jName = (**env).NewStringUTF.expect("non-null function pointer")(
                env,
                b"turbojpeg.progressive\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !jName.is_null() {
                let mut exception_2: jboolean = 0;
                jValue = (**env)
                    .CallStaticObjectMethod
                    .expect("non-null function pointer")(
                    env, cls, mid, jName
                ) as jstring;
                exception_2 = (**env).ExceptionCheck.expect("non-null function pointer")(env);
                if !jValue.is_null() && exception_2 == 0 && {
                    value = (**env)
                        .GetStringUTFChars
                        .expect("non-null function pointer")(
                        env,
                        jValue,
                        ::core::ptr::null_mut::<jboolean>(),
                    );
                    !value.is_null()
                } {
                    PUTENV_S(
                        b"TJ_PROGRESSIVE\0" as *const u8 as *const ::core::ffi::c_char,
                        value,
                    );
                    (**env)
                        .ReleaseStringUTFChars
                        .expect("non-null function pointer")(env, jValue, value);
                }
            }
            return 0 as ::core::ffi::c_int;
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_bufSize(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut width: jint,
    mut height: jint,
    mut jpegSubsamp: jint,
) -> jint {
    let mut retval: ::core::ffi::c_ulong = tjBufSize(
        width as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        jpegSubsamp as ::core::ffi::c_int,
    );
    if retval == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else if retval > INT_MAX as ::core::ffi::c_ulong {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return retval as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__IIII(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut width: jint,
    mut align: jint,
    mut height: jint,
    mut subsamp: jint,
) -> jint {
    let mut retval: ::core::ffi::c_ulong = tjBufSizeYUV2(
        width as ::core::ffi::c_int,
        align as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        subsamp as ::core::ffi::c_int,
    );
    if retval == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else if retval > INT_MAX as ::core::ffi::c_ulong {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return retval as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__III(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut width: jint,
    mut height: jint,
    mut subsamp: jint,
) -> jint {
    return Java_org_libjpegturbo_turbojpeg_TJ_bufSizeYUV__IIII(
        env, cls, width, 4 as jint, height, subsamp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_planeSizeYUV__IIIII(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut componentID: jint,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut subsamp: jint,
) -> jint {
    let mut retval: ::core::ffi::c_ulong = tjPlaneSizeYUV(
        componentID as ::core::ffi::c_int,
        width as ::core::ffi::c_int,
        stride as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        subsamp as ::core::ffi::c_int,
    );
    if retval == -(1 as ::core::ffi::c_int) as ::core::ffi::c_ulong {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else if retval > INT_MAX as ::core::ffi::c_ulong {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Image is too large\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    return retval as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_planeWidth__III(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut componentID: jint,
    mut width: jint,
    mut subsamp: jint,
) -> jint {
    let mut retval: jint = tjPlaneWidth(
        componentID as ::core::ffi::c_int,
        width as ::core::ffi::c_int,
        subsamp as ::core::ffi::c_int,
    );
    if retval == -(1 as ::core::ffi::c_int) {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_planeHeight__III(
    mut env: *mut JNIEnv,
    mut cls: jclass,
    mut componentID: jint,
    mut height: jint,
    mut subsamp: jint,
) -> jint {
    let mut retval: jint = tjPlaneHeight(
        componentID as ::core::ffi::c_int,
        height as ::core::ffi::c_int,
        subsamp as ::core::ffi::c_int,
    );
    if retval == -(1 as ::core::ffi::c_int) {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_init(
    mut env: *mut JNIEnv,
    mut obj: jobject,
) {
    let mut cls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    handle = tjInitCompress();
    if handle.is_null() {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else {
        cls = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
        if !(cls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            fid = (**env).GetFieldID.expect("non-null function pointer")(
                env,
                cls,
                b"handle\0" as *const u8 as *const ::core::ffi::c_char,
                b"J\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(fid.is_null()
                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                    as ::core::ffi::c_int
                    != 0)
            {
                (**env).SetLongField.expect("non-null function pointer")(
                    env,
                    obj,
                    fid,
                    handle as size_t as jlong,
                );
            }
        }
    };
}
unsafe extern "C" fn TJCompressor_compress(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jarray,
    mut srcElementSize: jint,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut jpegSubsamp: jint,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegSize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut arraySize: jsize = 0 as jsize;
    let mut actualPitch: jsize = 0;
    let mut srcBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if pf < 0 as ::core::ffi::c_int
                || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
                || width < 1 as ::core::ffi::c_int
                || height < 1 as ::core::ffi::c_int
                || pitch < 0 as ::core::ffi::c_int
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in compress()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMPF != TJ_NUMPF as ::core::ffi::c_long {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                actualPitch = (if pitch == 0 as ::core::ffi::c_int {
                    width as ::core::ffi::c_int * tjPixelSize[pf as usize]
                } else {
                    pitch as ::core::ffi::c_int
                }) as jsize;
                arraySize = ((y as ::core::ffi::c_int + height as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    * actualPitch as ::core::ffi::c_int
                    + (x as ::core::ffi::c_int + width as ::core::ffi::c_int)
                        * tjPixelSize[pf as usize]) as jsize;
                if (**env).GetArrayLength.expect("non-null function pointer")(env, src) as jint
                    * srcElementSize
                    < arraySize
                {
                    let mut _exccls_1: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_1.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_1,
                            b"Source buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    jpegSize = tjBufSize(
                        width as ::core::ffi::c_int,
                        height as ::core::ffi::c_int,
                        jpegSubsamp as ::core::ffi::c_int,
                    );
                    if (**env).GetArrayLength.expect("non-null function pointer")(
                        env,
                        dst as jarray,
                    ) < jpegSize as jsize
                    {
                        let mut _exccls_2: jclass =
                            (**env).FindClass.expect("non-null function pointer")(
                                env,
                                b"java/lang/IllegalArgumentException\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        if !(_exccls_2.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            (**env).ThrowNew.expect("non-null function pointer")(
                                env,
                                _exccls_2,
                                b"Destination buffer is not large enough\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                        }
                    } else if !(ProcessSystemProperties(env) < 0 as ::core::ffi::c_int) {
                        srcBuf = (**env)
                            .GetPrimitiveArrayCritical
                            .expect("non-null function pointer")(
                            env,
                            src,
                            ::core::ptr::null_mut::<jboolean>(),
                        ) as *mut ::core::ffi::c_uchar;
                        if !srcBuf.is_null() {
                            jpegBuf = (**env)
                                .GetPrimitiveArrayCritical
                                .expect("non-null function pointer")(
                                env,
                                dst as jarray,
                                ::core::ptr::null_mut::<jboolean>(),
                            ) as *mut ::core::ffi::c_uchar;
                            if !jpegBuf.is_null() {
                                if tjCompress2(
                                    handle,
                                    srcBuf.offset(
                                        (y as ::core::ffi::c_int
                                            * actualPitch as ::core::ffi::c_int
                                            + x as ::core::ffi::c_int
                                                * *(&raw const tjPixelSize
                                                    as *const ::core::ffi::c_int)
                                                    .offset(pf as isize))
                                            as isize,
                                    )
                                        as *mut ::core::ffi::c_uchar,
                                    width as ::core::ffi::c_int,
                                    pitch as ::core::ffi::c_int,
                                    height as ::core::ffi::c_int,
                                    pf as ::core::ffi::c_int,
                                    &raw mut jpegBuf,
                                    &raw mut jpegSize,
                                    jpegSubsamp as ::core::ffi::c_int,
                                    jpegQual as ::core::ffi::c_int,
                                    flags as ::core::ffi::c_int | TJFLAG_NOREALLOC,
                                ) == -(1 as ::core::ffi::c_int)
                                {
                                    if !dst.is_null() && !jpegBuf.is_null() {
                                        (**env)
                                            .ReleasePrimitiveArrayCritical
                                            .expect("non-null function pointer")(
                                            env,
                                            dst as jarray,
                                            jpegBuf as *mut ::core::ffi::c_void,
                                            0 as jint,
                                        );
                                    }
                                    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    if !src.is_null() && !srcBuf.is_null() {
                                        (**env)
                                            .ReleasePrimitiveArrayCritical
                                            .expect("non-null function pointer")(
                                            env,
                                            src,
                                            srcBuf as *mut ::core::ffi::c_void,
                                            0 as jint,
                                        );
                                    }
                                    srcBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                    let mut _exccls_3: jclass = ::core::ptr::null_mut::<_jobject>();
                                    let mut _excid: jmethodID =
                                        ::core::ptr::null_mut::<_jmethodID>();
                                    let mut _excobj: jobject = ::core::ptr::null_mut::<_jobject>();
                                    let mut _errstr: jstring = ::core::ptr::null_mut::<_jobject>();
                                    _errstr =
                                        (**env).NewStringUTF.expect("non-null function pointer")(
                                            env,
                                            tjGetErrorStr2(handle),
                                        );
                                    if !(_errstr.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        _exccls_3 = (**env)
                                            .FindClass
                                            .expect("non-null function pointer")(
                                            env,
                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        if !(_exccls_3.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0)
                                        {
                                            _excid = (**env)
                                                .GetMethodID
                                                .expect("non-null function pointer")(
                                                env,
                                                _exccls_3,
                                                b"<init>\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"(Ljava/lang/String;I)V\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            if !(_excid.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                _excobj = (**env)
                                                    .NewObject
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    _exccls_3,
                                                    _excid,
                                                    _errstr,
                                                    tjGetErrorCode(handle),
                                                );
                                                if !(_excobj.is_null()
                                                    || (**env)
                                                        .ExceptionCheck
                                                        .expect("non-null function pointer")(
                                                        env
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0)
                                                {
                                                    (**env)
                                                        .Throw
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        _excobj as jthrowable,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !src.is_null() && !srcBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src,
            srcBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    srcBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    return jpegSize as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIIIII_3BIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut jpegSubsamp: jint,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    return TJCompressor_compress(
        env,
        obj,
        src as jarray,
        1 as jint,
        x,
        y,
        width,
        pitch,
        height,
        pf,
        dst,
        jpegSubsamp,
        jpegQual,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3BIIII_3BIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut jpegSubsamp: jint,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    return TJCompressor_compress(
        env,
        obj,
        src as jarray,
        1 as jint,
        0 as jint,
        0 as jint,
        width,
        pitch,
        height,
        pf,
        dst,
        jpegSubsamp,
        jpegQual,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIIIII_3BIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jintArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut jpegSubsamp: jint,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in compress()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when compressing from an integer buffer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        return TJCompressor_compress(
            env,
            obj,
            src as jarray,
            ::core::mem::size_of::<jint>() as jint,
            x,
            y,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            dst,
            jpegSubsamp,
            jpegQual,
            flags,
        );
    }
    return 0 as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_compress___3IIIII_3BIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jintArray,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut jpegSubsamp: jint,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in compress()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when compressing from an integer buffer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        return TJCompressor_compress(
            env,
            obj,
            src as jarray,
            ::core::mem::size_of::<jint>() as jint,
            0 as jint,
            0 as jint,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            dst,
            jpegSubsamp,
            jpegQual,
            flags,
        );
    }
    return 0 as jint;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_compressFromYUV___3_3B_3II_3III_3BII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut srcobjs: jobjectArray,
    mut jSrcOffsets: jintArray,
    mut width: jint,
    mut jSrcStrides: jintArray,
    mut height: jint,
    mut subsamp: jint,
    mut dst: jbyteArray,
    mut jpegQual: jint,
    mut flags: jint,
) -> jint {
    let mut current_block: u64;
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegSize: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut jSrcPlanes: [jbyteArray; 3] = [
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
    ];
    let mut srcPlanesTmp: [*const ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ];
    let mut srcPlanes: [*const ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ];
    let mut srcOffsetsTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcStridesTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcOffsets: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcStrides: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut nc: ::core::ffi::c_int =
        if subsamp as ::core::ffi::c_long == org_libjpegturbo_turbojpeg_TJ_SAMP_GRAY {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
    let mut i: ::core::ffi::c_int = 0;
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if subsamp < 0 as ::core::ffi::c_int
                || subsamp as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMSAMP
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in compressFromYUV()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMSAMP != TJ_NUMSAMP as ::core::ffi::c_long {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                srcobjs as jarray,
            ) < nc
            {
                let mut _exccls_1: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_1.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_1,
                        b"Planes array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jSrcOffsets as jarray,
            ) < nc
            {
                let mut _exccls_2: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_2.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_2,
                        b"Offsets array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jSrcStrides as jarray,
            ) < nc
            {
                let mut _exccls_3: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_3.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_3,
                        b"Strides array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                jpegSize = tjBufSize(
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                    subsamp as ::core::ffi::c_int,
                );
                if (**env).GetArrayLength.expect("non-null function pointer")(env, dst as jarray)
                    < jpegSize as jsize
                {
                    let mut _exccls_4: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_4.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_4,
                            b"Destination buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else if !(ProcessSystemProperties(env) < 0 as ::core::ffi::c_int) {
                    (**env)
                        .GetIntArrayRegion
                        .expect("non-null function pointer")(
                        env,
                        jSrcOffsets,
                        0 as jsize,
                        nc as jsize,
                        &raw mut srcOffsetsTmp as *mut jint,
                    );
                    if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                        i = 0 as ::core::ffi::c_int;
                        while i < 3 as ::core::ffi::c_int {
                            srcOffsets[i as usize] =
                                srcOffsetsTmp[i as usize] as ::core::ffi::c_int;
                            i += 1;
                        }
                        (**env)
                            .GetIntArrayRegion
                            .expect("non-null function pointer")(
                            env,
                            jSrcStrides,
                            0 as jsize,
                            nc as jsize,
                            &raw mut srcStridesTmp as *mut jint,
                        );
                        if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                            i = 0 as ::core::ffi::c_int;
                            while i < 3 as ::core::ffi::c_int {
                                srcStrides[i as usize] =
                                    srcStridesTmp[i as usize] as ::core::ffi::c_int;
                                i += 1;
                            }
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i < nc) {
                                    current_block = 14648606000749551097;
                                    break;
                                }
                                let mut planeSize: ::core::ffi::c_int = tjPlaneSizeYUV(
                                    i,
                                    width as ::core::ffi::c_int,
                                    srcStrides[i as usize],
                                    height as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                )
                                    as ::core::ffi::c_int;
                                let mut pw: ::core::ffi::c_int = tjPlaneWidth(
                                    i,
                                    width as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                );
                                if planeSize < 0 as ::core::ffi::c_int
                                    || pw < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_5: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_5.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 2015423512699278234;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_5,
                                        tjGetErrorStr(),
                                    );
                                    current_block = 2015423512699278234;
                                    break;
                                } else if srcOffsets[i as usize] < 0 as ::core::ffi::c_int {
                                    let mut _exccls_6: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_6.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 2015423512699278234;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_6,
                                        b"Invalid argument in compressFromYUV()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 2015423512699278234;
                                    break;
                                } else if srcStrides[i as usize] < 0 as ::core::ffi::c_int
                                    && srcOffsets[i as usize] - planeSize + pw
                                        < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_7: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_7.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 2015423512699278234;
                                        break;
                                    }
                                    (**env)
                                        .ThrowNew
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        env,
                                        _exccls_7,
                                        b"Negative plane stride would cause memory to be accessed below plane boundary\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    current_block = 2015423512699278234;
                                    break;
                                } else {
                                    jSrcPlanes[i as usize] = (**env)
                                        .GetObjectArrayElement
                                        .expect("non-null function pointer")(
                                        env, srcobjs, i as jsize,
                                    )
                                        as jbyteArray;
                                    if jSrcPlanes[i as usize].is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 2015423512699278234;
                                        break;
                                    }
                                    if (**env).GetArrayLength.expect("non-null function pointer")(
                                        env,
                                        jSrcPlanes[i as usize],
                                    ) < srcOffsets[i as usize] + planeSize
                                    {
                                        let mut _exccls_8: jclass =
                                            (**env).FindClass.expect("non-null function pointer")(
                                                env,
                                                b"java/lang/IllegalArgumentException\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        if _exccls_8.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            current_block = 2015423512699278234;
                                            break;
                                        }
                                        (**env).ThrowNew.expect("non-null function pointer")(
                                            env,
                                            _exccls_8,
                                            b"Source plane is not large enough\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        current_block = 2015423512699278234;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                            }
                            match current_block {
                                2015423512699278234 => {}
                                _ => {
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(i < nc) {
                                            current_block = 16791665189521845338;
                                            break;
                                        }
                                        srcPlanesTmp[i as usize] = (**env)
                                            .GetPrimitiveArrayCritical
                                            .expect("non-null function pointer")(
                                            env,
                                            jSrcPlanes[i as usize],
                                            ::core::ptr::null_mut::<jboolean>(),
                                        )
                                            as *const ::core::ffi::c_uchar;
                                        if srcPlanesTmp[i as usize].is_null() {
                                            current_block = 2015423512699278234;
                                            break;
                                        }
                                        srcPlanes[i as usize] = (*(&raw mut srcPlanesTmp
                                            as *mut *const ::core::ffi::c_uchar)
                                            .offset(i as isize))
                                        .offset(
                                            *(&raw mut srcOffsets as *mut ::core::ffi::c_int)
                                                .offset(i as isize)
                                                as isize,
                                        )
                                            as *const ::core::ffi::c_uchar;
                                        i += 1;
                                    }
                                    match current_block {
                                        2015423512699278234 => {}
                                        _ => {
                                            jpegBuf = (**env)
                                                .GetPrimitiveArrayCritical
                                                .expect("non-null function pointer")(
                                                env,
                                                dst as jarray,
                                                ::core::ptr::null_mut::<jboolean>(),
                                            )
                                                as *mut ::core::ffi::c_uchar;
                                            if !jpegBuf.is_null() {
                                                if tjCompressFromYUVPlanes(
                                                    handle,
                                                    &raw mut srcPlanes
                                                        as *mut *const ::core::ffi::c_uchar,
                                                    width as ::core::ffi::c_int,
                                                    &raw mut srcStrides as *mut ::core::ffi::c_int,
                                                    height as ::core::ffi::c_int,
                                                    subsamp as ::core::ffi::c_int,
                                                    &raw mut jpegBuf,
                                                    &raw mut jpegSize,
                                                    jpegQual as ::core::ffi::c_int,
                                                    flags as ::core::ffi::c_int | TJFLAG_NOREALLOC,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    if !dst.is_null() && !jpegBuf.is_null() {
                                                        (**env)
                                                            .ReleasePrimitiveArrayCritical
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            dst as jarray,
                                                            jpegBuf as *mut ::core::ffi::c_void,
                                                            0 as jint,
                                                        );
                                                    }
                                                    jpegBuf = ::core::ptr::null_mut::<
                                                        ::core::ffi::c_uchar,
                                                    >(
                                                    );
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < nc {
                                                        if !jSrcPlanes[i as usize].is_null()
                                                            && !srcPlanesTmp[i as usize].is_null()
                                                        {
                                                            (**env)
                                                                .ReleasePrimitiveArrayCritical
                                                                .expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                env,
                                                                jSrcPlanes[i as usize],
                                                                srcPlanesTmp[i as usize]
                                                                    as *mut ::core::ffi::c_void,
                                                                0 as jint,
                                                            );
                                                        }
                                                        srcPlanesTmp[i as usize] =
                                                            ::core::ptr::null::<::core::ffi::c_uchar>(
                                                            );
                                                        i += 1;
                                                    }
                                                    let mut _exccls_9: jclass =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _excid: jmethodID =
                                                        ::core::ptr::null_mut::<_jmethodID>();
                                                    let mut _excobj: jobject =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _errstr: jstring =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    _errstr = (**env)
                                                        .NewStringUTF
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        tjGetErrorStr2(handle),
                                                    );
                                                    if !(_errstr.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        _exccls_9 = (**env)
                                                            .FindClass
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            env,
                                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if !(_exccls_9.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0)
                                                        {
                                                            _excid = (**env).GetMethodID.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                env,
                                                                _exccls_9,
                                                                b"<init>\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                b"(Ljava/lang/String;I)V\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                            );
                                                            if !(_excid.is_null()
                                                                || (**env).ExceptionCheck.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    != 0)
                                                            {
                                                                _excobj = (**env).NewObject.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env,
                                                                    _exccls_9,
                                                                    _excid,
                                                                    _errstr,
                                                                    tjGetErrorCode(handle),
                                                                );
                                                                if !(_excobj.is_null()
                                                                    || (**env)
                                                                        .ExceptionCheck
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        != 0)
                                                                {
                                                                    (**env).Throw.expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env, _excobj as jthrowable
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    i = 0 as ::core::ffi::c_int;
    while i < nc {
        if !jSrcPlanes[i as usize].is_null() && !srcPlanesTmp[i as usize].is_null() {
            (**env)
                .ReleasePrimitiveArrayCritical
                .expect("non-null function pointer")(
                env,
                jSrcPlanes[i as usize],
                srcPlanesTmp[i as usize] as *mut ::core::ffi::c_void,
                0 as jint,
            );
        }
        srcPlanesTmp[i as usize] = ::core::ptr::null::<::core::ffi::c_uchar>();
        i += 1;
    }
    return jpegSize as jint;
}
unsafe extern "C" fn TJCompressor_encodeYUV(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jarray,
    mut srcElementSize: jint,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dstobjs: jobjectArray,
    mut jDstOffsets: jintArray,
    mut jDstStrides: jintArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    let mut current_block: u64;
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arraySize: jsize = 0 as jsize;
    let mut actualPitch: jsize = 0;
    let mut srcBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jDstPlanes: [jbyteArray; 3] = [
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
    ];
    let mut dstPlanesTmp: [*mut ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    ];
    let mut dstPlanes: [*mut ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    ];
    let mut dstOffsetsTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstStridesTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstOffsets: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstStrides: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut nc: ::core::ffi::c_int =
        if subsamp as ::core::ffi::c_long == org_libjpegturbo_turbojpeg_TJ_SAMP_GRAY {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
    let mut i: ::core::ffi::c_int = 0;
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if pf < 0 as ::core::ffi::c_int
                || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
                || width < 1 as ::core::ffi::c_int
                || height < 1 as ::core::ffi::c_int
                || pitch < 0 as ::core::ffi::c_int
                || subsamp < 0 as ::core::ffi::c_int
                || subsamp as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMSAMP
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in encodeYUV()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMPF != TJ_NUMPF as ::core::ffi::c_long
                || org_libjpegturbo_turbojpeg_TJ_NUMSAMP != TJ_NUMSAMP as ::core::ffi::c_long
            {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                dstobjs as jarray,
            ) < nc
            {
                let mut _exccls_1: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_1.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_1,
                        b"Planes array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jDstOffsets as jarray,
            ) < nc
            {
                let mut _exccls_2: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_2.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_2,
                        b"Offsets array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jDstStrides as jarray,
            ) < nc
            {
                let mut _exccls_3: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_3.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_3,
                        b"Strides array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                actualPitch = (if pitch == 0 as ::core::ffi::c_int {
                    width as ::core::ffi::c_int * tjPixelSize[pf as usize]
                } else {
                    pitch as ::core::ffi::c_int
                }) as jsize;
                arraySize = ((y as ::core::ffi::c_int + height as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    * actualPitch as ::core::ffi::c_int
                    + (x as ::core::ffi::c_int + width as ::core::ffi::c_int)
                        * tjPixelSize[pf as usize]) as jsize;
                if (**env).GetArrayLength.expect("non-null function pointer")(env, src) as jint
                    * srcElementSize
                    < arraySize
                {
                    let mut _exccls_4: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_4.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_4,
                            b"Source buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    (**env)
                        .GetIntArrayRegion
                        .expect("non-null function pointer")(
                        env,
                        jDstOffsets,
                        0 as jsize,
                        nc as jsize,
                        &raw mut dstOffsetsTmp as *mut jint,
                    );
                    if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                        i = 0 as ::core::ffi::c_int;
                        while i < 3 as ::core::ffi::c_int {
                            dstOffsets[i as usize] =
                                dstOffsetsTmp[i as usize] as ::core::ffi::c_int;
                            i += 1;
                        }
                        (**env)
                            .GetIntArrayRegion
                            .expect("non-null function pointer")(
                            env,
                            jDstStrides,
                            0 as jsize,
                            nc as jsize,
                            &raw mut dstStridesTmp as *mut jint,
                        );
                        if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                            i = 0 as ::core::ffi::c_int;
                            while i < 3 as ::core::ffi::c_int {
                                dstStrides[i as usize] =
                                    dstStridesTmp[i as usize] as ::core::ffi::c_int;
                                i += 1;
                            }
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i < nc) {
                                    current_block = 18383263831861166299;
                                    break;
                                }
                                let mut planeSize: ::core::ffi::c_int = tjPlaneSizeYUV(
                                    i,
                                    width as ::core::ffi::c_int,
                                    dstStrides[i as usize],
                                    height as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                )
                                    as ::core::ffi::c_int;
                                let mut pw: ::core::ffi::c_int = tjPlaneWidth(
                                    i,
                                    width as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                );
                                if planeSize < 0 as ::core::ffi::c_int
                                    || pw < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_5: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_5.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 11499779640769786642;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_5,
                                        tjGetErrorStr(),
                                    );
                                    current_block = 11499779640769786642;
                                    break;
                                } else if dstOffsets[i as usize] < 0 as ::core::ffi::c_int {
                                    let mut _exccls_6: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_6.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 11499779640769786642;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_6,
                                        b"Invalid argument in encodeYUV()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 11499779640769786642;
                                    break;
                                } else if dstStrides[i as usize] < 0 as ::core::ffi::c_int
                                    && dstOffsets[i as usize] - planeSize + pw
                                        < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_7: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_7.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 11499779640769786642;
                                        break;
                                    }
                                    (**env)
                                        .ThrowNew
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        env,
                                        _exccls_7,
                                        b"Negative plane stride would cause memory to be accessed below plane boundary\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    current_block = 11499779640769786642;
                                    break;
                                } else {
                                    jDstPlanes[i as usize] = (**env)
                                        .GetObjectArrayElement
                                        .expect("non-null function pointer")(
                                        env, dstobjs, i as jsize,
                                    )
                                        as jbyteArray;
                                    if jDstPlanes[i as usize].is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 11499779640769786642;
                                        break;
                                    }
                                    if (**env).GetArrayLength.expect("non-null function pointer")(
                                        env,
                                        jDstPlanes[i as usize],
                                    ) < dstOffsets[i as usize] + planeSize
                                    {
                                        let mut _exccls_8: jclass =
                                            (**env).FindClass.expect("non-null function pointer")(
                                                env,
                                                b"java/lang/IllegalArgumentException\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        if _exccls_8.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            current_block = 11499779640769786642;
                                            break;
                                        }
                                        (**env).ThrowNew.expect("non-null function pointer")(
                                            env,
                                            _exccls_8,
                                            b"Destination plane is not large enough\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        current_block = 11499779640769786642;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                            }
                            match current_block {
                                11499779640769786642 => {}
                                _ => {
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(i < nc) {
                                            current_block = 12705158477165241210;
                                            break;
                                        }
                                        dstPlanesTmp[i as usize] = (**env)
                                            .GetPrimitiveArrayCritical
                                            .expect("non-null function pointer")(
                                            env,
                                            jDstPlanes[i as usize],
                                            ::core::ptr::null_mut::<jboolean>(),
                                        )
                                            as *mut ::core::ffi::c_uchar;
                                        if dstPlanesTmp[i as usize].is_null() {
                                            current_block = 11499779640769786642;
                                            break;
                                        }
                                        dstPlanes[i as usize] = (*(&raw mut dstPlanesTmp
                                            as *mut *mut ::core::ffi::c_uchar)
                                            .offset(i as isize))
                                        .offset(
                                            *(&raw mut dstOffsets as *mut ::core::ffi::c_int)
                                                .offset(i as isize)
                                                as isize,
                                        )
                                            as *mut ::core::ffi::c_uchar;
                                        i += 1;
                                    }
                                    match current_block {
                                        11499779640769786642 => {}
                                        _ => {
                                            srcBuf = (**env)
                                                .GetPrimitiveArrayCritical
                                                .expect("non-null function pointer")(
                                                env,
                                                src,
                                                ::core::ptr::null_mut::<jboolean>(),
                                            )
                                                as *mut ::core::ffi::c_uchar;
                                            if !srcBuf.is_null() {
                                                if tjEncodeYUVPlanes(
                                                    handle,
                                                    srcBuf.offset(
                                                        (y as ::core::ffi::c_int
                                                            * actualPitch as ::core::ffi::c_int
                                                            + x as ::core::ffi::c_int
                                                                * *(&raw const tjPixelSize
                                                                    as *const ::core::ffi::c_int)
                                                                    .offset(pf as isize))
                                                            as isize,
                                                    )
                                                        as *mut ::core::ffi::c_uchar,
                                                    width as ::core::ffi::c_int,
                                                    pitch as ::core::ffi::c_int,
                                                    height as ::core::ffi::c_int,
                                                    pf as ::core::ffi::c_int,
                                                    &raw mut dstPlanes
                                                        as *mut *mut ::core::ffi::c_uchar,
                                                    &raw mut dstStrides as *mut ::core::ffi::c_int,
                                                    subsamp as ::core::ffi::c_int,
                                                    flags as ::core::ffi::c_int,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    if !src.is_null() && !srcBuf.is_null() {
                                                        (**env)
                                                            .ReleasePrimitiveArrayCritical
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            src,
                                                            srcBuf as *mut ::core::ffi::c_void,
                                                            0 as jint,
                                                        );
                                                    }
                                                    srcBuf = ::core::ptr::null_mut::<
                                                        ::core::ffi::c_uchar,
                                                    >(
                                                    );
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < nc {
                                                        if !jDstPlanes[i as usize].is_null()
                                                            && !dstPlanesTmp[i as usize].is_null()
                                                        {
                                                            (**env)
                                                                .ReleasePrimitiveArrayCritical
                                                                .expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                env,
                                                                jDstPlanes[i as usize],
                                                                dstPlanesTmp[i as usize]
                                                                    as *mut ::core::ffi::c_void,
                                                                0 as jint,
                                                            );
                                                        }
                                                        dstPlanesTmp[i as usize] =
                                                            ::core::ptr::null_mut::<
                                                                ::core::ffi::c_uchar,
                                                            >(
                                                            );
                                                        i += 1;
                                                    }
                                                    let mut _exccls_9: jclass =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _excid: jmethodID =
                                                        ::core::ptr::null_mut::<_jmethodID>();
                                                    let mut _excobj: jobject =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _errstr: jstring =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    _errstr = (**env)
                                                        .NewStringUTF
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        tjGetErrorStr2(handle),
                                                    );
                                                    if !(_errstr.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        _exccls_9 = (**env)
                                                            .FindClass
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            env,
                                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if !(_exccls_9.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0)
                                                        {
                                                            _excid = (**env).GetMethodID.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                env,
                                                                _exccls_9,
                                                                b"<init>\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                b"(Ljava/lang/String;I)V\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                            );
                                                            if !(_excid.is_null()
                                                                || (**env).ExceptionCheck.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    != 0)
                                                            {
                                                                _excobj = (**env).NewObject.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env,
                                                                    _exccls_9,
                                                                    _excid,
                                                                    _errstr,
                                                                    tjGetErrorCode(handle),
                                                                );
                                                                if !(_excobj.is_null()
                                                                    || (**env)
                                                                        .ExceptionCheck
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        != 0)
                                                                {
                                                                    (**env).Throw.expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env, _excobj as jthrowable
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !src.is_null() && !srcBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src,
            srcBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    srcBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    i = 0 as ::core::ffi::c_int;
    while i < nc {
        if !jDstPlanes[i as usize].is_null() && !dstPlanesTmp[i as usize].is_null() {
            (**env)
                .ReleasePrimitiveArrayCritical
                .expect("non-null function pointer")(
                env,
                jDstPlanes[i as usize],
                dstPlanesTmp[i as usize] as *mut ::core::ffi::c_void,
                0 as jint,
            );
        }
        dstPlanesTmp[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIIIII_3_3B_3I_3III(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dstobjs: jobjectArray,
    mut jDstOffsets: jintArray,
    mut jDstStrides: jintArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    TJCompressor_encodeYUV(
        env,
        obj,
        src as jarray,
        1 as jint,
        x,
        y,
        width,
        pitch,
        height,
        pf,
        dstobjs,
        jDstOffsets,
        jDstStrides,
        subsamp,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIIIII_3_3B_3I_3III(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jintArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut dstobjs: jobjectArray,
    mut jDstOffsets: jintArray,
    mut jDstStrides: jintArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in encodeYUV()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when encoding from an integer buffer.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        TJCompressor_encodeYUV(
            env,
            obj,
            src as jarray,
            ::core::mem::size_of::<jint>() as jint,
            x,
            y,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            dstobjs,
            jDstOffsets,
            jDstStrides,
            subsamp,
            flags,
        );
    };
}
unsafe extern "C" fn TJCompressor_encodeYUV_12(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jarray,
    mut srcElementSize: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arraySize: jsize = 0 as jsize;
    let mut srcBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dstBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if pf < 0 as ::core::ffi::c_int
                || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
                || width < 1 as ::core::ffi::c_int
                || height < 1 as ::core::ffi::c_int
                || pitch < 0 as ::core::ffi::c_int
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in encodeYUV()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMPF != TJ_NUMPF as ::core::ffi::c_long {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                arraySize = (if pitch == 0 as ::core::ffi::c_int {
                    width as ::core::ffi::c_int
                        * tjPixelSize[pf as usize]
                        * height as ::core::ffi::c_int
                } else {
                    pitch as ::core::ffi::c_int * height as ::core::ffi::c_int
                }) as jsize;
                if (**env).GetArrayLength.expect("non-null function pointer")(env, src) as jint
                    * srcElementSize
                    < arraySize
                {
                    let mut _exccls_1: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_1.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_1,
                            b"Source buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else if (**env).GetArrayLength.expect("non-null function pointer")(
                    env,
                    dst as jarray,
                ) < tjBufSizeYUV(
                    width as ::core::ffi::c_int,
                    height as ::core::ffi::c_int,
                    subsamp as ::core::ffi::c_int,
                ) as jsize
                {
                    let mut _exccls_2: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_2.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_2,
                            b"Destination buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    srcBuf = (**env)
                        .GetPrimitiveArrayCritical
                        .expect("non-null function pointer")(
                        env,
                        src,
                        ::core::ptr::null_mut::<jboolean>(),
                    ) as *mut ::core::ffi::c_uchar;
                    if !srcBuf.is_null() {
                        dstBuf = (**env)
                            .GetPrimitiveArrayCritical
                            .expect("non-null function pointer")(
                            env,
                            dst as jarray,
                            ::core::ptr::null_mut::<jboolean>(),
                        ) as *mut ::core::ffi::c_uchar;
                        if !dstBuf.is_null() {
                            if tjEncodeYUV2(
                                handle,
                                srcBuf,
                                width as ::core::ffi::c_int,
                                pitch as ::core::ffi::c_int,
                                height as ::core::ffi::c_int,
                                pf as ::core::ffi::c_int,
                                dstBuf,
                                subsamp as ::core::ffi::c_int,
                                flags as ::core::ffi::c_int,
                            ) == -(1 as ::core::ffi::c_int)
                            {
                                if !dst.is_null() && !dstBuf.is_null() {
                                    (**env)
                                        .ReleasePrimitiveArrayCritical
                                        .expect("non-null function pointer")(
                                        env,
                                        dst as jarray,
                                        dstBuf as *mut ::core::ffi::c_void,
                                        0 as jint,
                                    );
                                }
                                dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                if !src.is_null() && !srcBuf.is_null() {
                                    (**env)
                                        .ReleasePrimitiveArrayCritical
                                        .expect("non-null function pointer")(
                                        env,
                                        src,
                                        srcBuf as *mut ::core::ffi::c_void,
                                        0 as jint,
                                    );
                                }
                                srcBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                let mut _exccls_3: jclass = ::core::ptr::null_mut::<_jobject>();
                                let mut _excid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
                                let mut _excobj: jobject = ::core::ptr::null_mut::<_jobject>();
                                let mut _errstr: jstring = ::core::ptr::null_mut::<_jobject>();
                                _errstr = (**env).NewStringUTF.expect("non-null function pointer")(
                                    env,
                                    tjGetErrorStr2(handle),
                                );
                                if !(_errstr.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    _exccls_3 =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if !(_exccls_3.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        _excid = (**env)
                                            .GetMethodID
                                            .expect("non-null function pointer")(
                                            env,
                                            _exccls_3,
                                            b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"(Ljava/lang/String;I)V\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        if !(_excid.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0)
                                        {
                                            _excobj = (**env)
                                                .NewObject
                                                .expect("non-null function pointer")(
                                                env,
                                                _exccls_3,
                                                _excid,
                                                _errstr,
                                                tjGetErrorCode(handle),
                                            );
                                            if !(_excobj.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                (**env).Throw.expect("non-null function pointer")(
                                                    env,
                                                    _excobj as jthrowable,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !dstBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst as jarray,
            dstBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !src.is_null() && !srcBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src,
            srcBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    srcBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3BIIII_3BII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    TJCompressor_encodeYUV_12(
        env,
        obj,
        src as jarray,
        1 as jint,
        width,
        pitch,
        height,
        pf,
        dst,
        subsamp,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_encodeYUV___3IIIII_3BII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jintArray,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut dst: jbyteArray,
    mut subsamp: jint,
    mut flags: jint,
) {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in encodeYUV()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when encoding from an integer buffer.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        TJCompressor_encodeYUV_12(
            env,
            obj,
            src as jarray,
            ::core::mem::size_of::<jint>() as jint,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            dst,
            subsamp,
            flags,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJCompressor_destroy(
    mut env: *mut JNIEnv,
    mut obj: jobject,
) {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if tjDestroy(handle) == -(1 as ::core::ffi::c_int) {
                let mut _exccls: jclass = ::core::ptr::null_mut::<_jobject>();
                let mut _excid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
                let mut _excobj: jobject = ::core::ptr::null_mut::<_jobject>();
                let mut _errstr: jstring = ::core::ptr::null_mut::<_jobject>();
                _errstr = (**env).NewStringUTF.expect("non-null function pointer")(
                    env,
                    tjGetErrorStr2(handle),
                );
                if !(_errstr.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    _exccls = (**env).FindClass.expect("non-null function pointer")(
                        env,
                        b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    if !(_exccls.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        _excid = (**env).GetMethodID.expect("non-null function pointer")(
                            env,
                            _exccls,
                            b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                            b"(Ljava/lang/String;I)V\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(_excid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            _excobj = (**env).NewObject.expect("non-null function pointer")(
                                env,
                                _exccls,
                                _excid,
                                _errstr,
                                tjGetErrorCode(handle),
                            );
                            if !(_excobj.is_null()
                                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                    as ::core::ffi::c_int
                                    != 0)
                            {
                                (**env).Throw.expect("non-null function pointer")(
                                    env,
                                    _excobj as jthrowable,
                                );
                            }
                        }
                    }
                }
            } else {
                (**env).SetLongField.expect("non-null function pointer")(
                    env, obj, _fid, 0 as jlong,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_init(
    mut env: *mut JNIEnv,
    mut obj: jobject,
) {
    let mut cls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    handle = tjInitDecompress();
    if handle.is_null() {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else {
        cls = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
        if !(cls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            fid = (**env).GetFieldID.expect("non-null function pointer")(
                env,
                cls,
                b"handle\0" as *const u8 as *const ::core::ffi::c_char,
                b"J\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(fid.is_null()
                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                    as ::core::ffi::c_int
                    != 0)
            {
                (**env).SetLongField.expect("non-null function pointer")(
                    env,
                    obj,
                    fid,
                    handle as size_t as jlong,
                );
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJ_getScalingFactors(
    mut env: *mut JNIEnv,
    mut cls: jclass,
) -> jobjectArray {
    let mut sfcls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    let mut sf: *mut tjscalingfactor = ::core::ptr::null_mut::<tjscalingfactor>();
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut sfobj: jobject = ::core::ptr::null_mut::<_jobject>();
    let mut sfjava: jobjectArray = ::core::ptr::null_mut::<_jobject>();
    sf = tjGetScalingFactors(&raw mut n);
    if sf.is_null() || n == 0 as ::core::ffi::c_int {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else {
        sfcls = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"org/libjpegturbo/turbojpeg/TJScalingFactor\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        if !(sfcls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            sfjava = (**env).NewObjectArray.expect("non-null function pointer")(
                env,
                n as jsize,
                sfcls,
                ::core::ptr::null_mut::<_jobject>(),
            );
            if !(sfjava.is_null()
                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                    as ::core::ffi::c_int
                    != 0)
            {
                i = 0 as ::core::ffi::c_int;
                while i < n {
                    sfobj = (**env).AllocObject.expect("non-null function pointer")(env, sfcls);
                    if sfobj.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0
                    {
                        break;
                    }
                    fid = (**env).GetFieldID.expect("non-null function pointer")(
                        env,
                        sfcls,
                        b"num\0" as *const u8 as *const ::core::ffi::c_char,
                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if fid.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0
                    {
                        break;
                    }
                    (**env).SetIntField.expect("non-null function pointer")(
                        env,
                        sfobj,
                        fid,
                        (*sf.offset(i as isize)).num as jint,
                    );
                    fid = (**env).GetFieldID.expect("non-null function pointer")(
                        env,
                        sfcls,
                        b"denom\0" as *const u8 as *const ::core::ffi::c_char,
                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if fid.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0
                    {
                        break;
                    }
                    (**env).SetIntField.expect("non-null function pointer")(
                        env,
                        sfobj,
                        fid,
                        (*sf.offset(i as isize)).denom as jint,
                    );
                    (**env)
                        .SetObjectArrayElement
                        .expect("non-null function pointer")(
                        env, sfjava, i as jsize, sfobj
                    );
                    i += 1;
                }
            }
        }
    }
    return sfjava;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressHeader(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
) {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegSubsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut jpegColorspace: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if (**env).GetArrayLength.expect("non-null function pointer")(env, src as jarray)
                < jpegSize
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Source buffer is not large enough\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                jpegBuf = (**env)
                    .GetPrimitiveArrayCritical
                    .expect("non-null function pointer")(
                    env,
                    src as jarray,
                    ::core::ptr::null_mut::<jboolean>(),
                ) as *mut ::core::ffi::c_uchar;
                if !jpegBuf.is_null() {
                    if tjDecompressHeader3(
                        handle,
                        jpegBuf,
                        jpegSize as ::core::ffi::c_ulong,
                        &raw mut width,
                        &raw mut height,
                        &raw mut jpegSubsamp,
                        &raw mut jpegColorspace,
                    ) == -(1 as ::core::ffi::c_int)
                    {
                        if !src.is_null() && !jpegBuf.is_null() {
                            (**env)
                                .ReleasePrimitiveArrayCritical
                                .expect("non-null function pointer")(
                                env,
                                src as jarray,
                                jpegBuf as *mut ::core::ffi::c_void,
                                0 as jint,
                            );
                        }
                        jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        let mut _exccls_0: jclass = ::core::ptr::null_mut::<_jobject>();
                        let mut _excid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
                        let mut _excobj: jobject = ::core::ptr::null_mut::<_jobject>();
                        let mut _errstr: jstring = ::core::ptr::null_mut::<_jobject>();
                        _errstr = (**env).NewStringUTF.expect("non-null function pointer")(
                            env,
                            tjGetErrorStr2(handle),
                        );
                        if !(_errstr.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            _exccls_0 = (**env).FindClass.expect("non-null function pointer")(
                                env,
                                b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            if !(_exccls_0.is_null()
                                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                    as ::core::ffi::c_int
                                    != 0)
                            {
                                _excid = (**env).GetMethodID.expect("non-null function pointer")(
                                    env,
                                    _exccls_0,
                                    b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"(Ljava/lang/String;I)V\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                );
                                if !(_excid.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    _excobj = (**env).NewObject.expect("non-null function pointer")(
                                        env,
                                        _exccls_0,
                                        _excid,
                                        _errstr,
                                        tjGetErrorCode(handle),
                                    );
                                    if !(_excobj.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        (**env).Throw.expect("non-null function pointer")(
                                            env,
                                            _excobj as jthrowable,
                                        );
                                    }
                                }
                            }
                        }
                    } else {
                        if !src.is_null() && !jpegBuf.is_null() {
                            (**env)
                                .ReleasePrimitiveArrayCritical
                                .expect("non-null function pointer")(
                                env,
                                src as jarray,
                                jpegBuf as *mut ::core::ffi::c_void,
                                0 as jint,
                            );
                        }
                        jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                        _fid = (**env).GetFieldID.expect("non-null function pointer")(
                            env,
                            _cls,
                            b"jpegSubsamp\0" as *const u8 as *const ::core::ffi::c_char,
                            b"I\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(_fid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            (**env).SetIntField.expect("non-null function pointer")(
                                env,
                                obj,
                                _fid,
                                jpegSubsamp as jint,
                            );
                            _fid = (**env).GetFieldID.expect("non-null function pointer")(
                                env,
                                _cls,
                                b"jpegColorspace\0" as *const u8 as *const ::core::ffi::c_char,
                                b"I\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            if _fid.is_null() {
                                (**env).ExceptionClear.expect("non-null function pointer")(env);
                            } else {
                                (**env).SetIntField.expect("non-null function pointer")(
                                    env,
                                    obj,
                                    _fid,
                                    jpegColorspace as jint,
                                );
                            }
                            _fid = (**env).GetFieldID.expect("non-null function pointer")(
                                env,
                                _cls,
                                b"jpegWidth\0" as *const u8 as *const ::core::ffi::c_char,
                                b"I\0" as *const u8 as *const ::core::ffi::c_char,
                            );
                            if !(_fid.is_null()
                                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                    as ::core::ffi::c_int
                                    != 0)
                            {
                                (**env).SetIntField.expect("non-null function pointer")(
                                    env,
                                    obj,
                                    _fid,
                                    width as jint,
                                );
                                _fid = (**env).GetFieldID.expect("non-null function pointer")(
                                    env,
                                    _cls,
                                    b"jpegHeight\0" as *const u8 as *const ::core::ffi::c_char,
                                    b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                );
                                if !(_fid.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    (**env).SetIntField.expect("non-null function pointer")(
                                        env,
                                        obj,
                                        _fid,
                                        height as jint,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !src.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
unsafe extern "C" fn TJDecompressor_decompress(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jarray,
    mut dstElementSize: jint,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arraySize: jsize = 0 as jsize;
    let mut actualPitch: jsize = 0;
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dstBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if pf < 0 as ::core::ffi::c_int
                || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in decompress()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMPF != TJ_NUMPF as ::core::ffi::c_long {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(env, src as jarray)
                < jpegSize
            {
                let mut _exccls_1: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_1.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_1,
                        b"Source buffer is not large enough\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                actualPitch = (if pitch == 0 as ::core::ffi::c_int {
                    width as ::core::ffi::c_int * tjPixelSize[pf as usize]
                } else {
                    pitch as ::core::ffi::c_int
                }) as jsize;
                arraySize = ((y as ::core::ffi::c_int + height as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    * actualPitch as ::core::ffi::c_int
                    + (x as ::core::ffi::c_int + width as ::core::ffi::c_int)
                        * tjPixelSize[pf as usize]) as jsize;
                if (**env).GetArrayLength.expect("non-null function pointer")(env, dst) as jint
                    * dstElementSize
                    < arraySize
                {
                    let mut _exccls_2: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_2.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_2,
                            b"Destination buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    jpegBuf = (**env)
                        .GetPrimitiveArrayCritical
                        .expect("non-null function pointer")(
                        env,
                        src as jarray,
                        ::core::ptr::null_mut::<jboolean>(),
                    ) as *mut ::core::ffi::c_uchar;
                    if !jpegBuf.is_null() {
                        dstBuf = (**env)
                            .GetPrimitiveArrayCritical
                            .expect("non-null function pointer")(
                            env,
                            dst,
                            ::core::ptr::null_mut::<jboolean>(),
                        ) as *mut ::core::ffi::c_uchar;
                        if !dstBuf.is_null() {
                            if tjDecompress2(
                                handle,
                                jpegBuf,
                                jpegSize as ::core::ffi::c_ulong,
                                dstBuf.offset(
                                    (y as ::core::ffi::c_int * actualPitch as ::core::ffi::c_int
                                        + x as ::core::ffi::c_int
                                            * *(&raw const tjPixelSize
                                                as *const ::core::ffi::c_int)
                                                .offset(pf as isize))
                                        as isize,
                                ) as *mut ::core::ffi::c_uchar,
                                width as ::core::ffi::c_int,
                                pitch as ::core::ffi::c_int,
                                height as ::core::ffi::c_int,
                                pf as ::core::ffi::c_int,
                                flags as ::core::ffi::c_int,
                            ) == -(1 as ::core::ffi::c_int)
                            {
                                if !dst.is_null() && !dstBuf.is_null() {
                                    (**env)
                                        .ReleasePrimitiveArrayCritical
                                        .expect("non-null function pointer")(
                                        env,
                                        dst,
                                        dstBuf as *mut ::core::ffi::c_void,
                                        0 as jint,
                                    );
                                }
                                dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                if !src.is_null() && !jpegBuf.is_null() {
                                    (**env)
                                        .ReleasePrimitiveArrayCritical
                                        .expect("non-null function pointer")(
                                        env,
                                        src as jarray,
                                        jpegBuf as *mut ::core::ffi::c_void,
                                        0 as jint,
                                    );
                                }
                                jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                let mut _exccls_3: jclass = ::core::ptr::null_mut::<_jobject>();
                                let mut _excid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
                                let mut _excobj: jobject = ::core::ptr::null_mut::<_jobject>();
                                let mut _errstr: jstring = ::core::ptr::null_mut::<_jobject>();
                                _errstr = (**env).NewStringUTF.expect("non-null function pointer")(
                                    env,
                                    tjGetErrorStr2(handle),
                                );
                                if !(_errstr.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    _exccls_3 =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if !(_exccls_3.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        _excid = (**env)
                                            .GetMethodID
                                            .expect("non-null function pointer")(
                                            env,
                                            _exccls_3,
                                            b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                                            b"(Ljava/lang/String;I)V\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        if !(_excid.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0)
                                        {
                                            _excobj = (**env)
                                                .NewObject
                                                .expect("non-null function pointer")(
                                                env,
                                                _exccls_3,
                                                _excid,
                                                _errstr,
                                                tjGetErrorCode(handle),
                                            );
                                            if !(_excobj.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                (**env).Throw.expect("non-null function pointer")(
                                                    env,
                                                    _excobj as jthrowable,
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !dstBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst,
            dstBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !src.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jbyteArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    TJDecompressor_decompress(
        env,
        obj,
        src,
        jpegSize,
        dst as jarray,
        1 as jint,
        x,
        y,
        width,
        pitch,
        height,
        pf,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3BIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jbyteArray,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    TJDecompressor_decompress(
        env,
        obj,
        src,
        jpegSize,
        dst as jarray,
        1 as jint,
        0 as jint,
        0 as jint,
        width,
        pitch,
        height,
        pf,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jintArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in decompress()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when decompressing to an integer buffer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        TJDecompressor_decompress(
            env,
            obj,
            src,
            jpegSize,
            dst as jarray,
            ::core::mem::size_of::<jint>() as jint,
            x,
            y,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            flags,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompress___3BI_3IIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jintArray,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in decompress()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when decompressing to an integer buffer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else {
        TJDecompressor_decompress(
            env,
            obj,
            src,
            jpegSize,
            dst as jarray,
            ::core::mem::size_of::<jint>() as jint,
            0 as jint,
            0 as jint,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            flags,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3_3B_3II_3III(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dstobjs: jobjectArray,
    mut jDstOffsets: jintArray,
    mut desiredWidth: jint,
    mut jDstStrides: jintArray,
    mut desiredHeight: jint,
    mut flags: jint,
) {
    let mut current_block: u64;
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jDstPlanes: [jbyteArray; 3] = [
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
    ];
    let mut dstPlanesTmp: [*mut ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    ];
    let mut dstPlanes: [*mut ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    ];
    let mut dstOffsetsTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstStridesTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstOffsets: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstStrides: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut jpegSubsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut jpegWidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut width: ::core::ffi::c_int = 0;
    let mut height: ::core::ffi::c_int = 0;
    let mut scaledWidth: ::core::ffi::c_int = 0;
    let mut scaledHeight: ::core::ffi::c_int = 0;
    let mut nsf: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sf: *mut tjscalingfactor = ::core::ptr::null_mut::<tjscalingfactor>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if (**env).GetArrayLength.expect("non-null function pointer")(env, src as jarray)
                < jpegSize
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Source buffer is not large enough\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                _fid = (**env).GetFieldID.expect("non-null function pointer")(
                    env,
                    _cls,
                    b"jpegSubsamp\0" as *const u8 as *const ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !(_fid.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    jpegSubsamp =
                        (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                    _fid = (**env).GetFieldID.expect("non-null function pointer")(
                        env,
                        _cls,
                        b"jpegWidth\0" as *const u8 as *const ::core::ffi::c_char,
                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(_fid.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        jpegWidth =
                            (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                        _fid = (**env).GetFieldID.expect("non-null function pointer")(
                            env,
                            _cls,
                            b"jpegHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            b"I\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(_fid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            jpegHeight = (**env).GetIntField.expect("non-null function pointer")(
                                env, obj, _fid,
                            );
                            nc = if jpegSubsamp as ::core::ffi::c_long
                                == org_libjpegturbo_turbojpeg_TJ_SAMP_GRAY
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                3 as ::core::ffi::c_int
                            };
                            width = desiredWidth as ::core::ffi::c_int;
                            height = desiredHeight as ::core::ffi::c_int;
                            if width == 0 as ::core::ffi::c_int {
                                width = jpegWidth;
                            }
                            if height == 0 as ::core::ffi::c_int {
                                height = jpegHeight;
                            }
                            sf = tjGetScalingFactors(&raw mut nsf);
                            if sf.is_null() || nsf < 1 as ::core::ffi::c_int {
                                let mut _exccls_0: jclass =
                                    (**env).FindClass.expect("non-null function pointer")(
                                        env,
                                        b"java/lang/IllegalArgumentException\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                if !(_exccls_0.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_0,
                                        tjGetErrorStr(),
                                    );
                                }
                            } else {
                                i = 0 as ::core::ffi::c_int;
                                while i < nsf {
                                    scaledWidth = (jpegWidth * (*sf.offset(i as isize)).num
                                        + (*sf.offset(i as isize)).denom
                                        - 1 as ::core::ffi::c_int)
                                        / (*sf.offset(i as isize)).denom;
                                    scaledHeight = (jpegHeight * (*sf.offset(i as isize)).num
                                        + (*sf.offset(i as isize)).denom
                                        - 1 as ::core::ffi::c_int)
                                        / (*sf.offset(i as isize)).denom;
                                    if scaledWidth <= width && scaledHeight <= height {
                                        break;
                                    }
                                    i += 1;
                                }
                                if i >= nsf {
                                    let mut _exccls_1: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if !(_exccls_1.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        (**env).ThrowNew.expect("non-null function pointer")(
                                            env,
                                            _exccls_1,
                                            b"Could not scale down to desired image dimensions\0"
                                                as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                } else {
                                    (**env)
                                        .GetIntArrayRegion
                                        .expect("non-null function pointer")(
                                        env,
                                        jDstOffsets,
                                        0 as jsize,
                                        nc as jsize,
                                        &raw mut dstOffsetsTmp as *mut jint,
                                    );
                                    if !((**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) != 0)
                                    {
                                        i = 0 as ::core::ffi::c_int;
                                        while i < 3 as ::core::ffi::c_int {
                                            dstOffsets[i as usize] =
                                                dstOffsetsTmp[i as usize] as ::core::ffi::c_int;
                                            i += 1;
                                        }
                                        (**env)
                                            .GetIntArrayRegion
                                            .expect("non-null function pointer")(
                                            env,
                                            jDstStrides,
                                            0 as jsize,
                                            nc as jsize,
                                            &raw mut dstStridesTmp as *mut jint,
                                        );
                                        if !((**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        ) != 0)
                                        {
                                            i = 0 as ::core::ffi::c_int;
                                            while i < 3 as ::core::ffi::c_int {
                                                dstStrides[i as usize] =
                                                    dstStridesTmp[i as usize] as ::core::ffi::c_int;
                                                i += 1;
                                            }
                                            i = 0 as ::core::ffi::c_int;
                                            loop {
                                                if !(i < nc) {
                                                    current_block = 5722677567366458307;
                                                    break;
                                                }
                                                let mut planeSize: ::core::ffi::c_int =
                                                    tjPlaneSizeYUV(
                                                        i,
                                                        scaledWidth,
                                                        dstStrides[i as usize],
                                                        scaledHeight,
                                                        jpegSubsamp,
                                                    )
                                                        as ::core::ffi::c_int;
                                                let mut pw: ::core::ffi::c_int =
                                                    tjPlaneWidth(i, scaledWidth, jpegSubsamp);
                                                if planeSize < 0 as ::core::ffi::c_int
                                                    || pw < 0 as ::core::ffi::c_int
                                                {
                                                    let mut _exccls_2: jclass = (**env)
                                                        .FindClass
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        b"java/lang/IllegalArgumentException\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if _exccls_2.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        current_block = 12510491367174741214;
                                                        break;
                                                    }
                                                    (**env)
                                                        .ThrowNew
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        _exccls_2,
                                                        tjGetErrorStr(),
                                                    );
                                                    current_block = 12510491367174741214;
                                                    break;
                                                } else if dstOffsets[i as usize]
                                                    < 0 as ::core::ffi::c_int
                                                {
                                                    let mut _exccls_3: jclass = (**env)
                                                        .FindClass
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        b"java/lang/IllegalArgumentException\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if _exccls_3.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        current_block = 12510491367174741214;
                                                        break;
                                                    }
                                                    (**env)
                                                        .ThrowNew
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        _exccls_3,
                                                        b"Invalid argument in decompressToYUV()\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    current_block = 12510491367174741214;
                                                    break;
                                                } else if dstStrides[i as usize]
                                                    < 0 as ::core::ffi::c_int
                                                    && dstOffsets[i as usize] - planeSize + pw
                                                        < 0 as ::core::ffi::c_int
                                                {
                                                    let mut _exccls_4: jclass = (**env)
                                                        .FindClass
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        b"java/lang/IllegalArgumentException\0"
                                                            as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if _exccls_4.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        current_block = 12510491367174741214;
                                                        break;
                                                    }
                                                    (**env)
                                                        .ThrowNew
                                                        .expect(
                                                            "non-null function pointer",
                                                        )(
                                                        env,
                                                        _exccls_4,
                                                        b"Negative plane stride would cause memory to be accessed below plane boundary\0"
                                                            as *const u8 as *const ::core::ffi::c_char,
                                                    );
                                                    current_block = 12510491367174741214;
                                                    break;
                                                } else {
                                                    jDstPlanes[i as usize] = (**env)
                                                        .GetObjectArrayElement
                                                        .expect("non-null function pointer")(
                                                        env, dstobjs, i as jsize,
                                                    )
                                                        as jbyteArray;
                                                    if jDstPlanes[i as usize].is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0
                                                    {
                                                        current_block = 12510491367174741214;
                                                        break;
                                                    }
                                                    if (**env)
                                                        .GetArrayLength
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        jDstPlanes[i as usize],
                                                    ) < dstOffsets[i as usize] + planeSize
                                                    {
                                                        let mut _exccls_5: jclass = (**env)
                                                            .FindClass
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            b"java/lang/IllegalArgumentException\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _exccls_5.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 12510491367174741214;
                                                            break;
                                                        }
                                                        (**env)
                                                            .ThrowNew
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            env,
                                                            _exccls_5,
                                                            b"Destination plane is not large enough\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        current_block = 12510491367174741214;
                                                        break;
                                                    } else {
                                                        i += 1;
                                                    }
                                                }
                                            }
                                            match current_block {
                                                12510491367174741214 => {}
                                                _ => {
                                                    i = 0 as ::core::ffi::c_int;
                                                    loop {
                                                        if !(i < nc) {
                                                            current_block = 6471821049853688503;
                                                            break;
                                                        }
                                                        dstPlanesTmp[i as usize] = (**env)
                                                            .GetPrimitiveArrayCritical
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            jDstPlanes[i as usize],
                                                            ::core::ptr::null_mut::<jboolean>(),
                                                        )
                                                            as *mut ::core::ffi::c_uchar;
                                                        if dstPlanesTmp[i as usize].is_null() {
                                                            current_block = 12510491367174741214;
                                                            break;
                                                        }
                                                        dstPlanes[i as usize] =
                                                            (*(&raw mut dstPlanesTmp
                                                                as *mut *mut ::core::ffi::c_uchar)
                                                                .offset(i as isize))
                                                            .offset(
                                                                *(&raw mut dstOffsets
                                                                    as *mut ::core::ffi::c_int)
                                                                    .offset(i as isize)
                                                                    as isize,
                                                            )
                                                                as *mut ::core::ffi::c_uchar;
                                                        i += 1;
                                                    }
                                                    match current_block {
                                                        12510491367174741214 => {}
                                                        _ => {
                                                            jpegBuf = (**env)
                                                                .GetPrimitiveArrayCritical
                                                                .expect("non-null function pointer")(
                                                                env,
                                                                src as jarray,
                                                                ::core::ptr::null_mut::<jboolean>(),
                                                            )
                                                                as *mut ::core::ffi::c_uchar;
                                                            if !jpegBuf.is_null() {
                                                                if tjDecompressToYUVPlanes(
                                                                    handle,
                                                                    jpegBuf,
                                                                    jpegSize as ::core::ffi::c_ulong,
                                                                    &raw mut dstPlanes as *mut *mut ::core::ffi::c_uchar,
                                                                    desiredWidth as ::core::ffi::c_int,
                                                                    &raw mut dstStrides as *mut ::core::ffi::c_int,
                                                                    desiredHeight as ::core::ffi::c_int,
                                                                    flags as ::core::ffi::c_int,
                                                                ) == -(1 as ::core::ffi::c_int)
                                                                {
                                                                    if !src.is_null() && !jpegBuf.is_null() {
                                                                        (**env)
                                                                            .ReleasePrimitiveArrayCritical
                                                                            .expect(
                                                                                "non-null function pointer",
                                                                            )(
                                                                            env,
                                                                            src as jarray,
                                                                            jpegBuf as *mut ::core::ffi::c_void,
                                                                            0 as jint,
                                                                        );
                                                                    }
                                                                    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                                    i = 0 as ::core::ffi::c_int;
                                                                    while i < nc {
                                                                        if !jDstPlanes[i as usize].is_null()
                                                                            && !dstPlanesTmp[i as usize].is_null()
                                                                        {
                                                                            (**env)
                                                                                .ReleasePrimitiveArrayCritical
                                                                                .expect(
                                                                                    "non-null function pointer",
                                                                                )(
                                                                                env,
                                                                                jDstPlanes[i as usize],
                                                                                dstPlanesTmp[i as usize] as *mut ::core::ffi::c_void,
                                                                                0 as jint,
                                                                            );
                                                                        }
                                                                        dstPlanesTmp[i as usize] = ::core::ptr::null_mut::<
                                                                            ::core::ffi::c_uchar,
                                                                        >();
                                                                        i += 1;
                                                                    }
                                                                    let mut _exccls_6: jclass = ::core::ptr::null_mut::<
                                                                        _jobject,
                                                                    >();
                                                                    let mut _excid: jmethodID = ::core::ptr::null_mut::<
                                                                        _jmethodID,
                                                                    >();
                                                                    let mut _excobj: jobject = ::core::ptr::null_mut::<
                                                                        _jobject,
                                                                    >();
                                                                    let mut _errstr: jstring = ::core::ptr::null_mut::<
                                                                        _jobject,
                                                                    >();
                                                                    _errstr = (**env)
                                                                        .NewStringUTF
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(env, tjGetErrorStr2(handle));
                                                                    if !(_errstr.is_null()
                                                                        || (**env)
                                                                            .ExceptionCheck
                                                                            .expect("non-null function pointer")(env)
                                                                            as ::core::ffi::c_int != 0)
                                                                    {
                                                                        _exccls_6 = (**env)
                                                                            .FindClass
                                                                            .expect(
                                                                                "non-null function pointer",
                                                                            )(
                                                                            env,
                                                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                                                as *const ::core::ffi::c_char,
                                                                        );
                                                                        if !(_exccls_6.is_null()
                                                                            || (**env)
                                                                                .ExceptionCheck
                                                                                .expect("non-null function pointer")(env)
                                                                                as ::core::ffi::c_int != 0)
                                                                        {
                                                                            _excid = (**env)
                                                                                .GetMethodID
                                                                                .expect(
                                                                                    "non-null function pointer",
                                                                                )(
                                                                                env,
                                                                                _exccls_6,
                                                                                b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                b"(Ljava/lang/String;I)V\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                            );
                                                                            if !(_excid.is_null()
                                                                                || (**env)
                                                                                    .ExceptionCheck
                                                                                    .expect("non-null function pointer")(env)
                                                                                    as ::core::ffi::c_int != 0)
                                                                            {
                                                                                _excobj = (**env)
                                                                                    .NewObject
                                                                                    .expect(
                                                                                        "non-null function pointer",
                                                                                    )(env, _exccls_6, _excid, _errstr, tjGetErrorCode(handle));
                                                                                if !(_excobj.is_null()
                                                                                    || (**env)
                                                                                        .ExceptionCheck
                                                                                        .expect("non-null function pointer")(env)
                                                                                        as ::core::ffi::c_int != 0)
                                                                                {
                                                                                    (**env)
                                                                                        .Throw
                                                                                        .expect(
                                                                                            "non-null function pointer",
                                                                                        )(env, _excobj as jthrowable);
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !src.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    i = 0 as ::core::ffi::c_int;
    while i < nc {
        if !jDstPlanes[i as usize].is_null() && !dstPlanesTmp[i as usize].is_null() {
            (**env)
                .ReleasePrimitiveArrayCritical
                .expect("non-null function pointer")(
                env,
                jDstPlanes[i as usize],
                dstPlanesTmp[i as usize] as *mut ::core::ffi::c_void,
                0 as jint,
            );
        }
        dstPlanesTmp[i as usize] = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decompressToYUV___3BI_3BI(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut src: jbyteArray,
    mut jpegSize: jint,
    mut dst: jbyteArray,
    mut flags: jint,
) {
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dstBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut jpegSubsamp: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut jpegWidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if (**env).GetArrayLength.expect("non-null function pointer")(env, src as jarray)
                < jpegSize
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Source buffer is not large enough\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                _fid = (**env).GetFieldID.expect("non-null function pointer")(
                    env,
                    _cls,
                    b"jpegSubsamp\0" as *const u8 as *const ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !(_fid.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    jpegSubsamp =
                        (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                    _fid = (**env).GetFieldID.expect("non-null function pointer")(
                        env,
                        _cls,
                        b"jpegWidth\0" as *const u8 as *const ::core::ffi::c_char,
                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(_fid.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        jpegWidth =
                            (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                        _fid = (**env).GetFieldID.expect("non-null function pointer")(
                            env,
                            _cls,
                            b"jpegHeight\0" as *const u8 as *const ::core::ffi::c_char,
                            b"I\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(_fid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            jpegHeight = (**env).GetIntField.expect("non-null function pointer")(
                                env, obj, _fid,
                            );
                            if (**env).GetArrayLength.expect("non-null function pointer")(
                                env,
                                dst as jarray,
                            ) < tjBufSizeYUV(jpegWidth, jpegHeight, jpegSubsamp) as jsize
                            {
                                let mut _exccls_0: jclass =
                                    (**env).FindClass.expect("non-null function pointer")(
                                        env,
                                        b"java/lang/IllegalArgumentException\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                if !(_exccls_0.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_0,
                                        b"Destination buffer is not large enough\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                }
                            } else {
                                jpegBuf = (**env)
                                    .GetPrimitiveArrayCritical
                                    .expect("non-null function pointer")(
                                    env,
                                    src as jarray,
                                    ::core::ptr::null_mut::<jboolean>(),
                                )
                                    as *mut ::core::ffi::c_uchar;
                                if !jpegBuf.is_null() {
                                    dstBuf = (**env)
                                        .GetPrimitiveArrayCritical
                                        .expect("non-null function pointer")(
                                        env,
                                        dst as jarray,
                                        ::core::ptr::null_mut::<jboolean>(),
                                    )
                                        as *mut ::core::ffi::c_uchar;
                                    if !dstBuf.is_null() {
                                        if tjDecompressToYUV(
                                            handle,
                                            jpegBuf,
                                            jpegSize as ::core::ffi::c_ulong,
                                            dstBuf,
                                            flags as ::core::ffi::c_int,
                                        ) == -(1 as ::core::ffi::c_int)
                                        {
                                            if !dst.is_null() && !dstBuf.is_null() {
                                                (**env)
                                                    .ReleasePrimitiveArrayCritical
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    dst as jarray,
                                                    dstBuf as *mut ::core::ffi::c_void,
                                                    0 as jint,
                                                );
                                            }
                                            dstBuf =
                                                ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                            if !src.is_null() && !jpegBuf.is_null() {
                                                (**env)
                                                    .ReleasePrimitiveArrayCritical
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    src as jarray,
                                                    jpegBuf as *mut ::core::ffi::c_void,
                                                    0 as jint,
                                                );
                                            }
                                            jpegBuf =
                                                ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                            let mut _exccls_1: jclass =
                                                ::core::ptr::null_mut::<_jobject>();
                                            let mut _excid: jmethodID =
                                                ::core::ptr::null_mut::<_jmethodID>();
                                            let mut _excobj: jobject =
                                                ::core::ptr::null_mut::<_jobject>();
                                            let mut _errstr: jstring =
                                                ::core::ptr::null_mut::<_jobject>();
                                            _errstr = (**env)
                                                .NewStringUTF
                                                .expect("non-null function pointer")(
                                                env,
                                                tjGetErrorStr2(handle),
                                            );
                                            if !(_errstr.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                _exccls_1 = (**env)
                                                    .FindClass
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    b"org/libjpegturbo/turbojpeg/TJException\0"
                                                        as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                if !(_exccls_1.is_null()
                                                    || (**env)
                                                        .ExceptionCheck
                                                        .expect("non-null function pointer")(
                                                        env
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0)
                                                {
                                                    _excid = (**env)
                                                        .GetMethodID
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        _exccls_1,
                                                        b"<init>\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        b"(Ljava/lang/String;I)V\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if !(_excid.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        _excobj = (**env)
                                                            .NewObject
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _exccls_1,
                                                            _excid,
                                                            _errstr,
                                                            tjGetErrorCode(handle),
                                                        );
                                                        if !(_excobj.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0)
                                                        {
                                                            (**env).Throw.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                env, _excobj as jthrowable
                                                            );
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !dstBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst as jarray,
            dstBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if !src.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            src as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
}
unsafe extern "C" fn TJDecompressor_decodeYUV(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut srcobjs: jobjectArray,
    mut jSrcOffsets: jintArray,
    mut jSrcStrides: jintArray,
    mut subsamp: jint,
    mut dst: jarray,
    mut dstElementSize: jint,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    let mut current_block: u64;
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut arraySize: jsize = 0 as jsize;
    let mut actualPitch: jsize = 0;
    let mut jSrcPlanes: [jbyteArray; 3] = [
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
        ::core::ptr::null_mut::<_jobject>(),
    ];
    let mut srcPlanesTmp: [*const ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ];
    let mut srcPlanes: [*const ::core::ffi::c_uchar; 3] = [
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ];
    let mut srcOffsetsTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcStridesTmp: [jint; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcOffsets: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut srcStrides: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    ];
    let mut dstBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut nc: ::core::ffi::c_int =
        if subsamp as ::core::ffi::c_long == org_libjpegturbo_turbojpeg_TJ_SAMP_GRAY {
            1 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        };
    let mut i: ::core::ffi::c_int = 0;
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if pf < 0 as ::core::ffi::c_int
                || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
                || subsamp < 0 as ::core::ffi::c_int
                || subsamp as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMSAMP
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Invalid argument in decodeYUV()\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if org_libjpegturbo_turbojpeg_TJ_NUMPF != TJ_NUMPF as ::core::ffi::c_long
                || org_libjpegturbo_turbojpeg_TJ_NUMSAMP != TJ_NUMSAMP as ::core::ffi::c_long
            {
                let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_0.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_0,
                        b"Mismatch between Java and C API\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                srcobjs as jarray,
            ) < nc
            {
                let mut _exccls_1: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_1.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_1,
                        b"Planes array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jSrcOffsets as jarray,
            ) < nc
            {
                let mut _exccls_2: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_2.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_2,
                        b"Offsets array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (**env).GetArrayLength.expect("non-null function pointer")(
                env,
                jSrcStrides as jarray,
            ) < nc
            {
                let mut _exccls_3: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls_3.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls_3,
                        b"Strides array is too small for the subsampling type\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                actualPitch = (if pitch == 0 as ::core::ffi::c_int {
                    width as ::core::ffi::c_int * tjPixelSize[pf as usize]
                } else {
                    pitch as ::core::ffi::c_int
                }) as jsize;
                arraySize = ((y as ::core::ffi::c_int + height as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    * actualPitch as ::core::ffi::c_int
                    + (x as ::core::ffi::c_int + width as ::core::ffi::c_int)
                        * tjPixelSize[pf as usize]) as jsize;
                if (**env).GetArrayLength.expect("non-null function pointer")(env, dst) as jint
                    * dstElementSize
                    < arraySize
                {
                    let mut _exccls_4: jclass =
                        (**env).FindClass.expect("non-null function pointer")(
                            env,
                            b"java/lang/IllegalArgumentException\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    if !(_exccls_4.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        (**env).ThrowNew.expect("non-null function pointer")(
                            env,
                            _exccls_4,
                            b"Destination buffer is not large enough\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                    }
                } else {
                    (**env)
                        .GetIntArrayRegion
                        .expect("non-null function pointer")(
                        env,
                        jSrcOffsets,
                        0 as jsize,
                        nc as jsize,
                        &raw mut srcOffsetsTmp as *mut jint,
                    );
                    if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                        i = 0 as ::core::ffi::c_int;
                        while i < 3 as ::core::ffi::c_int {
                            srcOffsets[i as usize] =
                                srcOffsetsTmp[i as usize] as ::core::ffi::c_int;
                            i += 1;
                        }
                        (**env)
                            .GetIntArrayRegion
                            .expect("non-null function pointer")(
                            env,
                            jSrcStrides,
                            0 as jsize,
                            nc as jsize,
                            &raw mut srcStridesTmp as *mut jint,
                        );
                        if !((**env).ExceptionCheck.expect("non-null function pointer")(env) != 0) {
                            i = 0 as ::core::ffi::c_int;
                            while i < 3 as ::core::ffi::c_int {
                                srcStrides[i as usize] =
                                    srcStridesTmp[i as usize] as ::core::ffi::c_int;
                                i += 1;
                            }
                            i = 0 as ::core::ffi::c_int;
                            loop {
                                if !(i < nc) {
                                    current_block = 18383263831861166299;
                                    break;
                                }
                                let mut planeSize: ::core::ffi::c_int = tjPlaneSizeYUV(
                                    i,
                                    width as ::core::ffi::c_int,
                                    srcStrides[i as usize],
                                    height as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                )
                                    as ::core::ffi::c_int;
                                let mut pw: ::core::ffi::c_int = tjPlaneWidth(
                                    i,
                                    width as ::core::ffi::c_int,
                                    subsamp as ::core::ffi::c_int,
                                );
                                if planeSize < 0 as ::core::ffi::c_int
                                    || pw < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_5: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_5.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 12498228244621828765;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_5,
                                        tjGetErrorStr(),
                                    );
                                    current_block = 12498228244621828765;
                                    break;
                                } else if srcOffsets[i as usize] < 0 as ::core::ffi::c_int {
                                    let mut _exccls_6: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_6.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 12498228244621828765;
                                        break;
                                    }
                                    (**env).ThrowNew.expect("non-null function pointer")(
                                        env,
                                        _exccls_6,
                                        b"Invalid argument in decodeYUV()\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    current_block = 12498228244621828765;
                                    break;
                                } else if srcStrides[i as usize] < 0 as ::core::ffi::c_int
                                    && srcOffsets[i as usize] - planeSize + pw
                                        < 0 as ::core::ffi::c_int
                                {
                                    let mut _exccls_7: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/IllegalArgumentException\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if _exccls_7.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 12498228244621828765;
                                        break;
                                    }
                                    (**env)
                                        .ThrowNew
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        env,
                                        _exccls_7,
                                        b"Negative plane stride would cause memory to be accessed below plane boundary\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                    current_block = 12498228244621828765;
                                    break;
                                } else {
                                    jSrcPlanes[i as usize] = (**env)
                                        .GetObjectArrayElement
                                        .expect("non-null function pointer")(
                                        env, srcobjs, i as jsize,
                                    )
                                        as jbyteArray;
                                    if jSrcPlanes[i as usize].is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0
                                    {
                                        current_block = 12498228244621828765;
                                        break;
                                    }
                                    if (**env).GetArrayLength.expect("non-null function pointer")(
                                        env,
                                        jSrcPlanes[i as usize],
                                    ) < srcOffsets[i as usize] + planeSize
                                    {
                                        let mut _exccls_8: jclass =
                                            (**env).FindClass.expect("non-null function pointer")(
                                                env,
                                                b"java/lang/IllegalArgumentException\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        if _exccls_8.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0
                                        {
                                            current_block = 12498228244621828765;
                                            break;
                                        }
                                        (**env).ThrowNew.expect("non-null function pointer")(
                                            env,
                                            _exccls_8,
                                            b"Source plane is not large enough\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                        current_block = 12498228244621828765;
                                        break;
                                    } else {
                                        i += 1;
                                    }
                                }
                            }
                            match current_block {
                                12498228244621828765 => {}
                                _ => {
                                    i = 0 as ::core::ffi::c_int;
                                    loop {
                                        if !(i < nc) {
                                            current_block = 12705158477165241210;
                                            break;
                                        }
                                        srcPlanesTmp[i as usize] = (**env)
                                            .GetPrimitiveArrayCritical
                                            .expect("non-null function pointer")(
                                            env,
                                            jSrcPlanes[i as usize],
                                            ::core::ptr::null_mut::<jboolean>(),
                                        )
                                            as *const ::core::ffi::c_uchar;
                                        if srcPlanesTmp[i as usize].is_null() {
                                            current_block = 12498228244621828765;
                                            break;
                                        }
                                        srcPlanes[i as usize] = (*(&raw mut srcPlanesTmp
                                            as *mut *const ::core::ffi::c_uchar)
                                            .offset(i as isize))
                                        .offset(
                                            *(&raw mut srcOffsets as *mut ::core::ffi::c_int)
                                                .offset(i as isize)
                                                as isize,
                                        )
                                            as *const ::core::ffi::c_uchar;
                                        i += 1;
                                    }
                                    match current_block {
                                        12498228244621828765 => {}
                                        _ => {
                                            dstBuf = (**env)
                                                .GetPrimitiveArrayCritical
                                                .expect("non-null function pointer")(
                                                env,
                                                dst,
                                                ::core::ptr::null_mut::<jboolean>(),
                                            )
                                                as *mut ::core::ffi::c_uchar;
                                            if !dstBuf.is_null() {
                                                if tjDecodeYUVPlanes(
                                                    handle,
                                                    &raw mut srcPlanes
                                                        as *mut *const ::core::ffi::c_uchar,
                                                    &raw mut srcStrides as *mut ::core::ffi::c_int,
                                                    subsamp as ::core::ffi::c_int,
                                                    dstBuf.offset(
                                                        (y as ::core::ffi::c_int
                                                            * actualPitch as ::core::ffi::c_int
                                                            + x as ::core::ffi::c_int
                                                                * *(&raw const tjPixelSize
                                                                    as *const ::core::ffi::c_int)
                                                                    .offset(pf as isize))
                                                            as isize,
                                                    )
                                                        as *mut ::core::ffi::c_uchar,
                                                    width as ::core::ffi::c_int,
                                                    pitch as ::core::ffi::c_int,
                                                    height as ::core::ffi::c_int,
                                                    pf as ::core::ffi::c_int,
                                                    flags as ::core::ffi::c_int,
                                                ) == -(1 as ::core::ffi::c_int)
                                                {
                                                    if !dst.is_null() && !dstBuf.is_null() {
                                                        (**env)
                                                            .ReleasePrimitiveArrayCritical
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            dst,
                                                            dstBuf as *mut ::core::ffi::c_void,
                                                            0 as jint,
                                                        );
                                                    }
                                                    dstBuf = ::core::ptr::null_mut::<
                                                        ::core::ffi::c_uchar,
                                                    >(
                                                    );
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < nc {
                                                        if !jSrcPlanes[i as usize].is_null()
                                                            && !srcPlanesTmp[i as usize].is_null()
                                                        {
                                                            (**env)
                                                                .ReleasePrimitiveArrayCritical
                                                                .expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                env,
                                                                jSrcPlanes[i as usize],
                                                                srcPlanesTmp[i as usize]
                                                                    as *mut ::core::ffi::c_void,
                                                                0 as jint,
                                                            );
                                                        }
                                                        srcPlanesTmp[i as usize] =
                                                            ::core::ptr::null::<::core::ffi::c_uchar>(
                                                            );
                                                        i += 1;
                                                    }
                                                    let mut _exccls_9: jclass =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _excid: jmethodID =
                                                        ::core::ptr::null_mut::<_jmethodID>();
                                                    let mut _excobj: jobject =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    let mut _errstr: jstring =
                                                        ::core::ptr::null_mut::<_jobject>();
                                                    _errstr = (**env)
                                                        .NewStringUTF
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        tjGetErrorStr2(handle),
                                                    );
                                                    if !(_errstr.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        _exccls_9 = (**env)
                                                            .FindClass
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            env,
                                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if !(_exccls_9.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0)
                                                        {
                                                            _excid = (**env).GetMethodID.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                env,
                                                                _exccls_9,
                                                                b"<init>\0" as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                                b"(Ljava/lang/String;I)V\0"
                                                                    as *const u8
                                                                    as *const ::core::ffi::c_char,
                                                            );
                                                            if !(_excid.is_null()
                                                                || (**env).ExceptionCheck.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    != 0)
                                                            {
                                                                _excobj = (**env).NewObject.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env,
                                                                    _exccls_9,
                                                                    _excid,
                                                                    _errstr,
                                                                    tjGetErrorCode(handle),
                                                                );
                                                                if !(_excobj.is_null()
                                                                    || (**env)
                                                                        .ExceptionCheck
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        != 0)
                                                                {
                                                                    (**env).Throw.expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env, _excobj as jthrowable
                                                                    );
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dst.is_null() && !dstBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            dst,
            dstBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    dstBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    i = 0 as ::core::ffi::c_int;
    while i < nc {
        if !jSrcPlanes[i as usize].is_null() && !srcPlanesTmp[i as usize].is_null() {
            (**env)
                .ReleasePrimitiveArrayCritical
                .expect("non-null function pointer")(
                env,
                jSrcPlanes[i as usize],
                srcPlanesTmp[i as usize] as *mut ::core::ffi::c_void,
                0 as jint,
            );
        }
        srcPlanesTmp[i as usize] = ::core::ptr::null::<::core::ffi::c_uchar>();
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3BIIIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut srcobjs: jobjectArray,
    mut jSrcOffsets: jintArray,
    mut jSrcStrides: jintArray,
    mut subsamp: jint,
    mut dst: jbyteArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut pitch: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    TJDecompressor_decodeYUV(
        env,
        obj,
        srcobjs,
        jSrcOffsets,
        jSrcStrides,
        subsamp,
        dst as jarray,
        1 as jint,
        x,
        y,
        width,
        pitch,
        height,
        pf,
        flags,
    );
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_decodeYUV___3_3B_3I_3II_3IIIIIIII(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut srcobjs: jobjectArray,
    mut jSrcOffsets: jintArray,
    mut jSrcStrides: jintArray,
    mut subsamp: jint,
    mut dst: jintArray,
    mut x: jint,
    mut y: jint,
    mut width: jint,
    mut stride: jint,
    mut height: jint,
    mut pf: jint,
    mut flags: jint,
) {
    if pf < 0 as ::core::ffi::c_int
        || pf as ::core::ffi::c_long >= org_libjpegturbo_turbojpeg_TJ_NUMPF
    {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls,
                b"Invalid argument in decodeYUV()\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    } else if tjPixelSize[pf as usize] as usize != ::core::mem::size_of::<jint>() as usize {
        let mut _exccls_0: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/lang/IllegalArgumentException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls_0.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(
                env,
                _exccls_0,
                b"Pixel format must be 32-bit when decoding to an integer buffer.\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
    } else {
        TJDecompressor_decodeYUV(
            env,
            obj,
            srcobjs,
            jSrcOffsets,
            jSrcStrides,
            subsamp,
            dst as jarray,
            ::core::mem::size_of::<jint>() as jint,
            x,
            y,
            width,
            (stride as usize).wrapping_mul(::core::mem::size_of::<jint>() as usize) as jint,
            height,
            pf,
            flags,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJTransformer_init(
    mut env: *mut JNIEnv,
    mut obj: jobject,
) {
    let mut cls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    handle = tjInitTransform();
    if handle.is_null() {
        let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_exccls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            (**env).ThrowNew.expect("non-null function pointer")(env, _exccls, tjGetErrorStr());
        }
    } else {
        cls = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
        if !(cls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            fid = (**env).GetFieldID.expect("non-null function pointer")(
                env,
                cls,
                b"handle\0" as *const u8 as *const ::core::ffi::c_char,
                b"J\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(fid.is_null()
                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                    as ::core::ffi::c_int
                    != 0)
            {
                (**env).SetLongField.expect("non-null function pointer")(
                    env,
                    obj,
                    fid,
                    handle as size_t as jlong,
                );
            }
        }
    };
}
unsafe extern "C" fn JNICustomFilter(
    mut coeffs: *mut ::core::ffi::c_short,
    mut arrayRegion: tjregion,
    mut planeRegion: tjregion,
    mut componentIndex: ::core::ffi::c_int,
    mut transformIndex: ::core::ffi::c_int,
    mut transform: *mut tjtransform,
) -> ::core::ffi::c_int {
    let mut params: *mut JNICustomFilterParams = (*transform).data as *mut JNICustomFilterParams;
    let mut env: *mut JNIEnv = (*params).env;
    let mut tobj: jobject = (*params).tobj;
    let mut cfobj: jobject = (*params).cfobj;
    let mut arrayRegionObj: jobject = ::core::ptr::null_mut::<_jobject>();
    let mut planeRegionObj: jobject = ::core::ptr::null_mut::<_jobject>();
    let mut bufobj: jobject = ::core::ptr::null_mut::<_jobject>();
    let mut borobj: jobject = ::core::ptr::null_mut::<_jobject>();
    let mut cls: jclass = ::core::ptr::null_mut::<_jobject>();
    let mut mid: jmethodID = ::core::ptr::null_mut::<_jmethodID>();
    let mut fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    bufobj = (**env)
        .NewDirectByteBuffer
        .expect("non-null function pointer")(
        env,
        coeffs as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<::core::ffi::c_short>() as usize)
            .wrapping_mul(arrayRegion.w as usize)
            .wrapping_mul(arrayRegion.h as usize) as jlong,
    );
    if !(bufobj.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        cls = (**env).FindClass.expect("non-null function pointer")(
            env,
            b"java/nio/ByteOrder\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(cls.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            mid = (**env)
                .GetStaticMethodID
                .expect("non-null function pointer")(
                env,
                cls,
                b"nativeOrder\0" as *const u8 as *const ::core::ffi::c_char,
                b"()Ljava/nio/ByteOrder;\0" as *const u8 as *const ::core::ffi::c_char,
            );
            if !(mid.is_null()
                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                    as ::core::ffi::c_int
                    != 0)
            {
                borobj = (**env)
                    .CallStaticObjectMethod
                    .expect("non-null function pointer")(env, cls, mid);
                if !(borobj.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    cls = (**env).GetObjectClass.expect("non-null function pointer")(env, bufobj);
                    if !(cls.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        mid = (**env).GetMethodID.expect("non-null function pointer")(
                            env,
                            cls,
                            b"order\0" as *const u8 as *const ::core::ffi::c_char,
                            b"(Ljava/nio/ByteOrder;)Ljava/nio/ByteBuffer;\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        if !(mid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            (**env).CallObjectMethod.expect("non-null function pointer")(
                                env, bufobj, mid, borobj,
                            );
                            mid = (**env).GetMethodID.expect("non-null function pointer")(
                                env,
                                cls,
                                b"asShortBuffer\0" as *const u8 as *const ::core::ffi::c_char,
                                b"()Ljava/nio/ShortBuffer;\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            if !(mid.is_null()
                                || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                    as ::core::ffi::c_int
                                    != 0)
                            {
                                bufobj =
                                    (**env).CallObjectMethod.expect("non-null function pointer")(
                                        env, bufobj, mid,
                                    );
                                if !(bufobj.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    cls = (**env).FindClass.expect("non-null function pointer")(
                                        env,
                                        b"java/awt/Rectangle\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                    if !(cls.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        arrayRegionObj = (**env)
                                            .AllocObject
                                            .expect("non-null function pointer")(
                                            env, cls
                                        );
                                        if !(arrayRegionObj.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0)
                                        {
                                            fid = (**env)
                                                .GetFieldID
                                                .expect("non-null function pointer")(
                                                env,
                                                cls,
                                                b"x\0" as *const u8 as *const ::core::ffi::c_char,
                                                b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                            );
                                            if !(fid.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                (**env)
                                                    .SetIntField
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    arrayRegionObj,
                                                    fid,
                                                    arrayRegion.x as jint,
                                                );
                                                fid = (**env)
                                                    .GetFieldID
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    cls,
                                                    b"y\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                    b"I\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                if !(fid.is_null()
                                                    || (**env)
                                                        .ExceptionCheck
                                                        .expect("non-null function pointer")(
                                                        env
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0)
                                                {
                                                    (**env)
                                                        .SetIntField
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        arrayRegionObj,
                                                        fid,
                                                        arrayRegion.y as jint,
                                                    );
                                                    fid = (**env)
                                                        .GetFieldID
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        cls,
                                                        b"width\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                        b"I\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if !(fid.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        (**env)
                                                            .SetIntField
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            arrayRegionObj,
                                                            fid,
                                                            arrayRegion.w as jint,
                                                        );
                                                        fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            cls,
                                                            b"height\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if !(fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0)
                                                        {
                                                            (**env).SetIntField.expect(
                                                                "non-null function pointer",
                                                            )(
                                                                env,
                                                                arrayRegionObj,
                                                                fid,
                                                                arrayRegion.h as jint,
                                                            );
                                                            planeRegionObj =
                                                                (**env).AllocObject.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env, cls
                                                                );
                                                            if !(planeRegionObj.is_null()
                                                                || (**env).ExceptionCheck.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env
                                                                )
                                                                    as ::core::ffi::c_int
                                                                    != 0)
                                                            {
                                                                fid = (**env)
                                                                    .GetFieldID
                                                                    .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                    env,
                                                                    cls,
                                                                    b"x\0" as *const u8 as *const ::core::ffi::c_char,
                                                                    b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                                                );
                                                                if !(fid.is_null()
                                                                    || (**env)
                                                                        .ExceptionCheck
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        != 0)
                                                                {
                                                                    (**env).SetIntField.expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env,
                                                                        planeRegionObj,
                                                                        fid,
                                                                        planeRegion.x as jint,
                                                                    );
                                                                    fid = (**env)
                                                                        .GetFieldID
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        env,
                                                                        cls,
                                                                        b"y\0" as *const u8 as *const ::core::ffi::c_char,
                                                                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                                                    );
                                                                    if !(fid.is_null()
                                                                        || (**env)
                                                                            .ExceptionCheck
                                                                            .expect("non-null function pointer")(env)
                                                                            as ::core::ffi::c_int != 0)
                                                                    {
                                                                        (**env)
                                                                            .SetIntField
                                                                            .expect(
                                                                                "non-null function pointer",
                                                                            )(env, planeRegionObj, fid, planeRegion.y as jint);
                                                                        fid = (**env)
                                                                            .GetFieldID
                                                                            .expect(
                                                                                "non-null function pointer",
                                                                            )(
                                                                            env,
                                                                            cls,
                                                                            b"width\0" as *const u8 as *const ::core::ffi::c_char,
                                                                            b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                                                        );
                                                                        if !(fid.is_null()
                                                                            || (**env)
                                                                                .ExceptionCheck
                                                                                .expect("non-null function pointer")(env)
                                                                                as ::core::ffi::c_int != 0)
                                                                        {
                                                                            (**env)
                                                                                .SetIntField
                                                                                .expect(
                                                                                    "non-null function pointer",
                                                                                )(env, planeRegionObj, fid, planeRegion.w as jint);
                                                                            fid = (**env)
                                                                                .GetFieldID
                                                                                .expect(
                                                                                    "non-null function pointer",
                                                                                )(
                                                                                env,
                                                                                cls,
                                                                                b"height\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                b"I\0" as *const u8 as *const ::core::ffi::c_char,
                                                                            );
                                                                            if !(fid.is_null()
                                                                                || (**env)
                                                                                    .ExceptionCheck
                                                                                    .expect("non-null function pointer")(env)
                                                                                    as ::core::ffi::c_int != 0)
                                                                            {
                                                                                (**env)
                                                                                    .SetIntField
                                                                                    .expect(
                                                                                        "non-null function pointer",
                                                                                    )(env, planeRegionObj, fid, planeRegion.h as jint);
                                                                                cls = (**env)
                                                                                    .GetObjectClass
                                                                                    .expect("non-null function pointer")(env, cfobj);
                                                                                if !(cls.is_null()
                                                                                    || (**env)
                                                                                        .ExceptionCheck
                                                                                        .expect("non-null function pointer")(env)
                                                                                        as ::core::ffi::c_int != 0)
                                                                                {
                                                                                    mid = (**env)
                                                                                        .GetMethodID
                                                                                        .expect(
                                                                                            "non-null function pointer",
                                                                                        )(
                                                                                        env,
                                                                                        cls,
                                                                                        b"customFilter\0" as *const u8
                                                                                            as *const ::core::ffi::c_char,
                                                                                        b"(Ljava/nio/ShortBuffer;Ljava/awt/Rectangle;Ljava/awt/Rectangle;IILorg/libjpegturbo/turbojpeg/TJTransform;)V\0"
                                                                                            as *const u8 as *const ::core::ffi::c_char,
                                                                                    );
                                                                                    if !(mid.is_null()
                                                                                        || (**env)
                                                                                            .ExceptionCheck
                                                                                            .expect("non-null function pointer")(env)
                                                                                            as ::core::ffi::c_int != 0)
                                                                                    {
                                                                                        (**env)
                                                                                            .CallVoidMethod
                                                                                            .expect(
                                                                                                "non-null function pointer",
                                                                                            )(
                                                                                            env,
                                                                                            cfobj,
                                                                                            mid,
                                                                                            bufobj,
                                                                                            arrayRegionObj,
                                                                                            planeRegionObj,
                                                                                            componentIndex,
                                                                                            transformIndex,
                                                                                            tobj,
                                                                                        );
                                                                                        return 0 as ::core::ffi::c_int;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJTransformer_transform(
    mut env: *mut JNIEnv,
    mut obj: jobject,
    mut jsrcBuf: jbyteArray,
    mut jpegSize: jint,
    mut dstobjs: jobjectArray,
    mut tobjs: jobjectArray,
    mut flags: jint,
) -> jintArray {
    let mut current_block: u64;
    let mut handle: tjhandle = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut jpegBuf: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut dstBufs: *mut *mut ::core::ffi::c_uchar =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>();
    let mut n: jsize = 0 as jsize;
    let mut dstSizes: *mut ::core::ffi::c_ulong = ::core::ptr::null_mut::<::core::ffi::c_ulong>();
    let mut t: *mut tjtransform = ::core::ptr::null_mut::<tjtransform>();
    let mut jdstBufs: *mut jbyteArray = ::core::ptr::null_mut::<jbyteArray>();
    let mut i: ::core::ffi::c_int = 0;
    let mut jpegWidth: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegHeight: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut jpegSubsamp: ::core::ffi::c_int = 0;
    let mut jdstSizes: jintArray = ::core::ptr::null_mut::<_jobject>();
    let mut dstSizesi: *mut jint = ::core::ptr::null_mut::<jint>();
    let mut params: *mut JNICustomFilterParams = ::core::ptr::null_mut::<JNICustomFilterParams>();
    let mut _cls: jclass = (**env).GetObjectClass.expect("non-null function pointer")(env, obj);
    let mut _fid: jfieldID = ::core::ptr::null_mut::<_jfieldID>();
    if !(_cls.is_null()
        || (**env).ExceptionCheck.expect("non-null function pointer")(env) as ::core::ffi::c_int
            != 0)
    {
        _fid = (**env).GetFieldID.expect("non-null function pointer")(
            env,
            _cls,
            b"handle\0" as *const u8 as *const ::core::ffi::c_char,
            b"J\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(_fid.is_null()
            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                as ::core::ffi::c_int
                != 0)
        {
            handle = (**env).GetLongField.expect("non-null function pointer")(env, obj, _fid)
                as size_t as tjhandle;
            if (**env).GetArrayLength.expect("non-null function pointer")(env, jsrcBuf as jarray)
                < jpegSize
            {
                let mut _exccls: jclass = (**env).FindClass.expect("non-null function pointer")(
                    env,
                    b"java/lang/IllegalArgumentException\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if !(_exccls.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    (**env).ThrowNew.expect("non-null function pointer")(
                        env,
                        _exccls,
                        b"Source buffer is not large enough\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else {
                _fid = (**env).GetFieldID.expect("non-null function pointer")(
                    env,
                    _cls,
                    b"jpegWidth\0" as *const u8 as *const ::core::ffi::c_char,
                    b"I\0" as *const u8 as *const ::core::ffi::c_char,
                );
                if !(_fid.is_null()
                    || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                        as ::core::ffi::c_int
                        != 0)
                {
                    jpegWidth =
                        (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                    _fid = (**env).GetFieldID.expect("non-null function pointer")(
                        env,
                        _cls,
                        b"jpegHeight\0" as *const u8 as *const ::core::ffi::c_char,
                        b"I\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    if !(_fid.is_null()
                        || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                            as ::core::ffi::c_int
                            != 0)
                    {
                        jpegHeight =
                            (**env).GetIntField.expect("non-null function pointer")(env, obj, _fid);
                        _fid = (**env).GetFieldID.expect("non-null function pointer")(
                            env,
                            _cls,
                            b"jpegSubsamp\0" as *const u8 as *const ::core::ffi::c_char,
                            b"I\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        if !(_fid.is_null()
                            || (**env).ExceptionCheck.expect("non-null function pointer")(env)
                                as ::core::ffi::c_int
                                != 0)
                        {
                            jpegSubsamp = (**env).GetIntField.expect("non-null function pointer")(
                                env, obj, _fid,
                            );
                            n = (**env).GetArrayLength.expect("non-null function pointer")(
                                env,
                                dstobjs as jarray,
                            );
                            if n != (**env).GetArrayLength.expect("non-null function pointer")(
                                env,
                                tobjs as jarray,
                            ) {
                                let mut _exccls_0: jclass =
                                    (**env).FindClass.expect("non-null function pointer")(
                                        env,
                                        b"java/lang/IllegalArgumentException\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                    );
                                if !(_exccls_0.is_null()
                                    || (**env).ExceptionCheck.expect("non-null function pointer")(
                                        env,
                                    ) as ::core::ffi::c_int
                                        != 0)
                                {
                                    (**env)
                                        .ThrowNew
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        env,
                                        _exccls_0,
                                        b"Mismatch between size of transforms array and destination buffers array\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                    );
                                }
                            } else {
                                dstBufs = malloc(
                                    (::core::mem::size_of::<*mut ::core::ffi::c_uchar>() as size_t)
                                        .wrapping_mul(n as size_t),
                                )
                                    as *mut *mut ::core::ffi::c_uchar;
                                if dstBufs.is_null() {
                                    let mut _exccls_1: jclass =
                                        (**env).FindClass.expect("non-null function pointer")(
                                            env,
                                            b"java/lang/OutOfMemoryError\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    if !(_exccls_1.is_null()
                                        || (**env)
                                            .ExceptionCheck
                                            .expect("non-null function pointer")(
                                            env
                                        )
                                            as ::core::ffi::c_int
                                            != 0)
                                    {
                                        (**env).ThrowNew.expect("non-null function pointer")(
                                            env,
                                            _exccls_1,
                                            b"Memory allocation failure\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                } else {
                                    jdstBufs = malloc(
                                        (::core::mem::size_of::<jbyteArray>() as size_t)
                                            .wrapping_mul(n as size_t),
                                    )
                                        as *mut jbyteArray;
                                    if jdstBufs.is_null() {
                                        let mut _exccls_2: jclass =
                                            (**env).FindClass.expect("non-null function pointer")(
                                                env,
                                                b"java/lang/OutOfMemoryError\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        if !(_exccls_2.is_null()
                                            || (**env)
                                                .ExceptionCheck
                                                .expect("non-null function pointer")(
                                                env
                                            )
                                                as ::core::ffi::c_int
                                                != 0)
                                        {
                                            (**env).ThrowNew.expect("non-null function pointer")(
                                                env,
                                                _exccls_2,
                                                b"Memory allocation failure\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                        }
                                    } else {
                                        dstSizes = malloc(
                                            (::core::mem::size_of::<::core::ffi::c_ulong>()
                                                as size_t)
                                                .wrapping_mul(n as size_t),
                                        )
                                            as *mut ::core::ffi::c_ulong;
                                        if dstSizes.is_null() {
                                            let mut _exccls_3: jclass = (**env)
                                                .FindClass
                                                .expect("non-null function pointer")(
                                                env,
                                                b"java/lang/OutOfMemoryError\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            );
                                            if !(_exccls_3.is_null()
                                                || (**env)
                                                    .ExceptionCheck
                                                    .expect("non-null function pointer")(
                                                    env
                                                )
                                                    as ::core::ffi::c_int
                                                    != 0)
                                            {
                                                (**env)
                                                    .ThrowNew
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    _exccls_3,
                                                    b"Memory allocation failure\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                            }
                                        } else {
                                            t = malloc(
                                                (::core::mem::size_of::<tjtransform>() as size_t)
                                                    .wrapping_mul(n as size_t),
                                            )
                                                as *mut tjtransform;
                                            if t.is_null() {
                                                let mut _exccls_4: jclass = (**env)
                                                    .FindClass
                                                    .expect("non-null function pointer")(
                                                    env,
                                                    b"java/lang/OutOfMemoryError\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                );
                                                if !(_exccls_4.is_null()
                                                    || (**env)
                                                        .ExceptionCheck
                                                        .expect("non-null function pointer")(
                                                        env
                                                    )
                                                        as ::core::ffi::c_int
                                                        != 0)
                                                {
                                                    (**env)
                                                        .ThrowNew
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        _exccls_4,
                                                        b"Memory allocation failure\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                }
                                            } else {
                                                params = malloc(
                                                    (::core::mem::size_of::<JNICustomFilterParams>() as size_t)
                                                        .wrapping_mul(n as size_t),
                                                ) as *mut JNICustomFilterParams;
                                                if params.is_null() {
                                                    let mut _exccls_5: jclass = (**env)
                                                        .FindClass
                                                        .expect("non-null function pointer")(
                                                        env,
                                                        b"java/lang/OutOfMemoryError\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                    if !(_exccls_5.is_null()
                                                        || (**env)
                                                            .ExceptionCheck
                                                            .expect("non-null function pointer")(
                                                            env,
                                                        )
                                                            as ::core::ffi::c_int
                                                            != 0)
                                                    {
                                                        (**env)
                                                            .ThrowNew
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _exccls_5,
                                                            b"Memory allocation failure\0"
                                                                as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                    }
                                                } else {
                                                    i = 0 as ::core::ffi::c_int;
                                                    while i < n {
                                                        let ref mut fresh1 =
                                                            *dstBufs.offset(i as isize);
                                                        *fresh1 = ::core::ptr::null_mut::<
                                                            ::core::ffi::c_uchar,
                                                        >(
                                                        );
                                                        let ref mut fresh2 =
                                                            *jdstBufs.offset(i as isize);
                                                        *fresh2 =
                                                            ::core::ptr::null_mut::<_jobject>();
                                                        *dstSizes.offset(i as isize) =
                                                            0 as ::core::ffi::c_ulong;
                                                        memset(
                                                            t.offset(i as isize) as *mut tjtransform
                                                                as *mut ::core::ffi::c_void,
                                                            0 as ::core::ffi::c_int,
                                                            ::core::mem::size_of::<tjtransform>()
                                                                as size_t,
                                                        );
                                                        memset(
                                                            params.offset(i as isize)
                                                                as *mut JNICustomFilterParams
                                                                as *mut ::core::ffi::c_void,
                                                            0 as ::core::ffi::c_int,
                                                            ::core::mem::size_of::<
                                                                JNICustomFilterParams,
                                                            >(
                                                            )
                                                                as size_t,
                                                        );
                                                        i += 1;
                                                    }
                                                    i = 0 as ::core::ffi::c_int;
                                                    loop {
                                                        if !(i < n) {
                                                            current_block = 9521147444787763968;
                                                            break;
                                                        }
                                                        let mut tobj: jobject =
                                                            ::core::ptr::null_mut::<_jobject>();
                                                        let mut cfobj: jobject =
                                                            ::core::ptr::null_mut::<_jobject>();
                                                        tobj = (**env)
                                                            .GetObjectArrayElement
                                                            .expect("non-null function pointer")(
                                                            env, tobjs, i as jsize,
                                                        );
                                                        if tobj.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        _cls = (**env)
                                                            .GetObjectClass
                                                            .expect("non-null function pointer")(
                                                            env, tobj,
                                                        );
                                                        if _cls.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"op\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).op = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"options\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).options = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"x\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).r.x = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"y\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).r.y = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"width\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).r.w = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect("non-null function pointer")(
                                                            env,
                                                            _cls,
                                                            b"height\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                            b"I\0" as *const u8
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        (*t.offset(i as isize)).r.h = (**env)
                                                            .GetIntField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        )
                                                            as ::core::ffi::c_int;
                                                        _fid = (**env)
                                                            .GetFieldID
                                                            .expect(
                                                                "non-null function pointer",
                                                            )(
                                                            env,
                                                            _cls,
                                                            b"cf\0" as *const u8 as *const ::core::ffi::c_char,
                                                            b"Lorg/libjpegturbo/turbojpeg/TJCustomFilter;\0"
                                                                as *const u8 as *const ::core::ffi::c_char,
                                                        );
                                                        if _fid.is_null()
                                                            || (**env)
                                                                .ExceptionCheck
                                                                .expect("non-null function pointer")(
                                                                env,
                                                            )
                                                                as ::core::ffi::c_int
                                                                != 0
                                                        {
                                                            current_block = 7657699556529250325;
                                                            break;
                                                        }
                                                        cfobj = (**env)
                                                            .GetObjectField
                                                            .expect("non-null function pointer")(
                                                            env, tobj, _fid,
                                                        );
                                                        if !cfobj.is_null() {
                                                            let ref mut fresh3 =
                                                                (*params.offset(i as isize)).env;
                                                            *fresh3 = env;
                                                            let ref mut fresh4 =
                                                                (*params.offset(i as isize)).tobj;
                                                            *fresh4 = tobj;
                                                            let ref mut fresh5 =
                                                                (*params.offset(i as isize)).cfobj;
                                                            *fresh5 = cfobj;
                                                            let ref mut fresh6 = (*t
                                                                .offset(i as isize))
                                                            .customFilter;
                                                            *fresh6 = Some(
                                                                JNICustomFilter
                                                                    as unsafe extern "C" fn(
                                                                        *mut ::core::ffi::c_short,
                                                                        tjregion,
                                                                        tjregion,
                                                                        ::core::ffi::c_int,
                                                                        ::core::ffi::c_int,
                                                                        *mut tjtransform,
                                                                    ) -> ::core::ffi::c_int,
                                                            )
                                                                as Option<
                                                                    unsafe extern "C" fn(
                                                                        *mut ::core::ffi::c_short,
                                                                        tjregion,
                                                                        tjregion,
                                                                        ::core::ffi::c_int,
                                                                        ::core::ffi::c_int,
                                                                        *mut tjtransform,
                                                                    ) -> ::core::ffi::c_int,
                                                                >;
                                                            let ref mut fresh7 =
                                                                (*t.offset(i as isize)).data;
                                                            *fresh7 = params.offset(i as isize)
                                                                as *mut JNICustomFilterParams
                                                                as *mut ::core::ffi::c_void;
                                                        }
                                                        i += 1;
                                                    }
                                                    match current_block {
                                                        7657699556529250325 => {}
                                                        _ => {
                                                            i = 0 as ::core::ffi::c_int;
                                                            loop {
                                                                if !(i < n) {
                                                                    current_block =
                                                                        17808209642927821499;
                                                                    break;
                                                                }
                                                                let mut w: ::core::ffi::c_int =
                                                                    jpegWidth;
                                                                let mut h: ::core::ffi::c_int =
                                                                    jpegHeight;
                                                                if (*t.offset(i as isize)).r.w
                                                                    != 0 as ::core::ffi::c_int
                                                                {
                                                                    w = (*t.offset(i as isize)).r.w;
                                                                }
                                                                if (*t.offset(i as isize)).r.h
                                                                    != 0 as ::core::ffi::c_int
                                                                {
                                                                    h = (*t.offset(i as isize)).r.h;
                                                                }
                                                                let ref mut fresh8 =
                                                                    *jdstBufs.offset(i as isize);
                                                                *fresh8 = (**env)
                                                                    .GetObjectArrayElement
                                                                    .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                    env, dstobjs, i as jsize
                                                                )
                                                                    as jbyteArray;
                                                                if (*fresh8).is_null()
                                                                    || (**env)
                                                                        .ExceptionCheck
                                                                        .expect(
                                                                        "non-null function pointer",
                                                                    )(
                                                                        env
                                                                    )
                                                                        as ::core::ffi::c_int
                                                                        != 0
                                                                {
                                                                    current_block =
                                                                        7657699556529250325;
                                                                    break;
                                                                }
                                                                if ((**env).GetArrayLength.expect(
                                                                    "non-null function pointer",
                                                                )(
                                                                    env,
                                                                    *jdstBufs.offset(i as isize)
                                                                        as jarray,
                                                                )
                                                                    as ::core::ffi::c_ulong)
                                                                    < tjBufSize(w, h, jpegSubsamp)
                                                                {
                                                                    let mut _exccls_6: jclass = (**env)
                                                                        .FindClass
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        env,
                                                                        b"java/lang/IllegalArgumentException\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    if _exccls_6.is_null()
                                                                        || (**env)
                                                                            .ExceptionCheck
                                                                            .expect("non-null function pointer")(env)
                                                                            as ::core::ffi::c_int != 0
                                                                    {
                                                                        current_block = 7657699556529250325;
                                                                        break;
                                                                    }
                                                                    (**env)
                                                                        .ThrowNew
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        env,
                                                                        _exccls_6,
                                                                        b"Destination buffer is not large enough\0" as *const u8
                                                                            as *const ::core::ffi::c_char,
                                                                    );
                                                                    current_block =
                                                                        7657699556529250325;
                                                                    break;
                                                                } else {
                                                                    i += 1;
                                                                }
                                                            }
                                                            match current_block {
                                                                7657699556529250325 => {}
                                                                _ => {
                                                                    jpegBuf = (**env)
                                                                        .GetPrimitiveArrayCritical
                                                                        .expect(
                                                                            "non-null function pointer",
                                                                        )(
                                                                        env,
                                                                        jsrcBuf as jarray,
                                                                        ::core::ptr::null_mut::<jboolean>(),
                                                                    ) as *mut ::core::ffi::c_uchar;
                                                                    if !jpegBuf.is_null() {
                                                                        i = 0 as ::core::ffi::c_int;
                                                                        loop {
                                                                            if !(i < n) {
                                                                                current_block = 17736998403848444560;
                                                                                break;
                                                                            }
                                                                            let ref mut fresh9 =
                                                                                *dstBufs.offset(
                                                                                    i as isize,
                                                                                );
                                                                            *fresh9 = (**env)
                                                                                .GetPrimitiveArrayCritical
                                                                                .expect(
                                                                                    "non-null function pointer",
                                                                                )(
                                                                                env,
                                                                                *jdstBufs.offset(i as isize) as jarray,
                                                                                ::core::ptr::null_mut::<jboolean>(),
                                                                            ) as *mut ::core::ffi::c_uchar;
                                                                            if (*fresh9).is_null() {
                                                                                current_block = 7657699556529250325;
                                                                                break;
                                                                            }
                                                                            i += 1;
                                                                        }
                                                                        match current_block {
                                                                            7657699556529250325 => {}
                                                                            _ => {
                                                                                if tjTransform(
                                                                                    handle,
                                                                                    jpegBuf,
                                                                                    jpegSize as ::core::ffi::c_ulong,
                                                                                    n as ::core::ffi::c_int,
                                                                                    dstBufs,
                                                                                    dstSizes,
                                                                                    t,
                                                                                    flags as ::core::ffi::c_int | TJFLAG_NOREALLOC,
                                                                                ) == -(1 as ::core::ffi::c_int)
                                                                                {
                                                                                    i = 0 as ::core::ffi::c_int;
                                                                                    while i < n {
                                                                                        if !(*jdstBufs.offset(i as isize)).is_null()
                                                                                            && !(*dstBufs.offset(i as isize)).is_null()
                                                                                        {
                                                                                            (**env)
                                                                                                .ReleasePrimitiveArrayCritical
                                                                                                .expect(
                                                                                                    "non-null function pointer",
                                                                                                )(
                                                                                                env,
                                                                                                *jdstBufs.offset(i as isize) as jarray,
                                                                                                *dstBufs.offset(i as isize) as *mut ::core::ffi::c_void,
                                                                                                0 as jint,
                                                                                            );
                                                                                        }
                                                                                        let ref mut fresh10 = *dstBufs.offset(i as isize);
                                                                                        *fresh10 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                                                        i += 1;
                                                                                    }
                                                                                    if !jsrcBuf.is_null() && !jpegBuf.is_null() {
                                                                                        (**env)
                                                                                            .ReleasePrimitiveArrayCritical
                                                                                            .expect(
                                                                                                "non-null function pointer",
                                                                                            )(
                                                                                            env,
                                                                                            jsrcBuf as jarray,
                                                                                            jpegBuf as *mut ::core::ffi::c_void,
                                                                                            0 as jint,
                                                                                        );
                                                                                    }
                                                                                    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                                                    let mut _exccls_7: jclass = ::core::ptr::null_mut::<
                                                                                        _jobject,
                                                                                    >();
                                                                                    let mut _excid: jmethodID = ::core::ptr::null_mut::<
                                                                                        _jmethodID,
                                                                                    >();
                                                                                    let mut _excobj: jobject = ::core::ptr::null_mut::<
                                                                                        _jobject,
                                                                                    >();
                                                                                    let mut _errstr: jstring = ::core::ptr::null_mut::<
                                                                                        _jobject,
                                                                                    >();
                                                                                    _errstr = (**env)
                                                                                        .NewStringUTF
                                                                                        .expect(
                                                                                            "non-null function pointer",
                                                                                        )(env, tjGetErrorStr2(handle));
                                                                                    if !(_errstr.is_null()
                                                                                        || (**env)
                                                                                            .ExceptionCheck
                                                                                            .expect("non-null function pointer")(env)
                                                                                            as ::core::ffi::c_int != 0)
                                                                                    {
                                                                                        _exccls_7 = (**env)
                                                                                            .FindClass
                                                                                            .expect(
                                                                                                "non-null function pointer",
                                                                                            )(
                                                                                            env,
                                                                                            b"org/libjpegturbo/turbojpeg/TJException\0" as *const u8
                                                                                                as *const ::core::ffi::c_char,
                                                                                        );
                                                                                        if !(_exccls_7.is_null()
                                                                                            || (**env)
                                                                                                .ExceptionCheck
                                                                                                .expect("non-null function pointer")(env)
                                                                                                as ::core::ffi::c_int != 0)
                                                                                        {
                                                                                            _excid = (**env)
                                                                                                .GetMethodID
                                                                                                .expect(
                                                                                                    "non-null function pointer",
                                                                                                )(
                                                                                                env,
                                                                                                _exccls_7,
                                                                                                b"<init>\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                b"(Ljava/lang/String;I)V\0" as *const u8
                                                                                                    as *const ::core::ffi::c_char,
                                                                                            );
                                                                                            if !(_excid.is_null()
                                                                                                || (**env)
                                                                                                    .ExceptionCheck
                                                                                                    .expect("non-null function pointer")(env)
                                                                                                    as ::core::ffi::c_int != 0)
                                                                                            {
                                                                                                _excobj = (**env)
                                                                                                    .NewObject
                                                                                                    .expect(
                                                                                                        "non-null function pointer",
                                                                                                    )(env, _exccls_7, _excid, _errstr, tjGetErrorCode(handle));
                                                                                                if !(_excobj.is_null()
                                                                                                    || (**env)
                                                                                                        .ExceptionCheck
                                                                                                        .expect("non-null function pointer")(env)
                                                                                                        as ::core::ffi::c_int != 0)
                                                                                                {
                                                                                                    (**env)
                                                                                                        .Throw
                                                                                                        .expect(
                                                                                                            "non-null function pointer",
                                                                                                        )(env, _excobj as jthrowable);
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    i = 0 as ::core::ffi::c_int;
                                                                                    while i < n {
                                                                                        if !(*jdstBufs.offset(i as isize)).is_null()
                                                                                            && !(*dstBufs.offset(i as isize)).is_null()
                                                                                        {
                                                                                            (**env)
                                                                                                .ReleasePrimitiveArrayCritical
                                                                                                .expect(
                                                                                                    "non-null function pointer",
                                                                                                )(
                                                                                                env,
                                                                                                *jdstBufs.offset(i as isize) as jarray,
                                                                                                *dstBufs.offset(i as isize) as *mut ::core::ffi::c_void,
                                                                                                0 as jint,
                                                                                            );
                                                                                        }
                                                                                        let ref mut fresh11 = *dstBufs.offset(i as isize);
                                                                                        *fresh11 = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                                                        i += 1;
                                                                                    }
                                                                                    if !jsrcBuf.is_null() && !jpegBuf.is_null() {
                                                                                        (**env)
                                                                                            .ReleasePrimitiveArrayCritical
                                                                                            .expect(
                                                                                                "non-null function pointer",
                                                                                            )(
                                                                                            env,
                                                                                            jsrcBuf as jarray,
                                                                                            jpegBuf as *mut ::core::ffi::c_void,
                                                                                            0 as jint,
                                                                                        );
                                                                                    }
                                                                                    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
                                                                                    jdstSizes = (**env)
                                                                                        .NewIntArray
                                                                                        .expect("non-null function pointer")(env, n);
                                                                                    dstSizesi = (**env)
                                                                                        .GetIntArrayElements
                                                                                        .expect(
                                                                                            "non-null function pointer",
                                                                                        )(env, jdstSizes, ::core::ptr::null_mut::<jboolean>());
                                                                                    if !(dstSizesi.is_null()
                                                                                        || (**env)
                                                                                            .ExceptionCheck
                                                                                            .expect("non-null function pointer")(env)
                                                                                            as ::core::ffi::c_int != 0)
                                                                                    {
                                                                                        i = 0 as ::core::ffi::c_int;
                                                                                        while i < n {
                                                                                            *dstSizesi.offset(i as isize) = *dstSizes.offset(i as isize)
                                                                                                as ::core::ffi::c_int as jint;
                                                                                            i += 1;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !dstSizesi.is_null() {
        (**env)
            .ReleaseIntArrayElements
            .expect("non-null function pointer")(env, jdstSizes, dstSizesi, 0 as jint);
    }
    if !dstBufs.is_null() {
        i = 0 as ::core::ffi::c_int;
        while i < n {
            if !(*dstBufs.offset(i as isize)).is_null()
                && !jdstBufs.is_null()
                && !(*jdstBufs.offset(i as isize)).is_null()
            {
                (**env)
                    .ReleasePrimitiveArrayCritical
                    .expect("non-null function pointer")(
                    env,
                    *jdstBufs.offset(i as isize) as jarray,
                    *dstBufs.offset(i as isize) as *mut ::core::ffi::c_void,
                    0 as jint,
                );
            }
            i += 1;
        }
        free(dstBufs as *mut ::core::ffi::c_void);
    }
    if !jsrcBuf.is_null() && !jpegBuf.is_null() {
        (**env)
            .ReleasePrimitiveArrayCritical
            .expect("non-null function pointer")(
            env,
            jsrcBuf as jarray,
            jpegBuf as *mut ::core::ffi::c_void,
            0 as jint,
        );
    }
    jpegBuf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    free(jdstBufs as *mut ::core::ffi::c_void);
    free(dstSizes as *mut ::core::ffi::c_void);
    free(t as *mut ::core::ffi::c_void);
    return jdstSizes;
}
#[no_mangle]
pub unsafe extern "C" fn Java_org_libjpegturbo_turbojpeg_TJDecompressor_destroy(
    mut env: *mut JNIEnv,
    mut obj: jobject,
) {
    Java_org_libjpegturbo_turbojpeg_TJCompressor_destroy(env, obj);
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
