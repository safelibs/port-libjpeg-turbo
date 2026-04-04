#[repr(C)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_barray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jvirt_sarray_control {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_entropy_encoder {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_forward_dct {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_downsampler {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_color_converter {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_marker_writer {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_coef_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_prep_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_c_main_controller {
    _unused: [u8; 0],
}
#[repr(C)]
pub struct jpeg_comp_master {
    _unused: [u8; 0],
}
extern "C" {
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn jpeg_default_qtables(cinfo: j_compress_ptr, force_baseline: boolean);
    fn jpeg_add_quant_table(
        cinfo: j_compress_ptr,
        which_tbl: ::core::ffi::c_int,
        basic_table: *const ::core::ffi::c_uint,
        scale_factor: ::core::ffi::c_int,
        force_baseline: boolean,
    );
    fn jpeg_quality_scaling(quality: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
}
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type JSAMPLE = ::core::ffi::c_uchar;
pub type JCOEF = ::core::ffi::c_short;
pub type JOCTET = ::core::ffi::c_uchar;
pub type UINT8 = ::core::ffi::c_uchar;
pub type UINT16 = ::core::ffi::c_ushort;
pub type JDIMENSION = ::core::ffi::c_uint;
pub type boolean = ::core::ffi::c_int;
pub type JSAMPROW = *mut JSAMPLE;
pub type JSAMPARRAY = *mut JSAMPROW;
pub type JBLOCK = [JCOEF; 64];
pub type JBLOCKROW = *mut JBLOCK;
pub type JBLOCKARRAY = *mut JBLOCKROW;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JQUANT_TBL {
    pub quantval: [UINT16; 64],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct JHUFF_TBL {
    pub bits: [UINT8; 17],
    pub huffval: [UINT8; 256],
    pub sent_table: boolean,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_component_info {
    pub component_id: ::core::ffi::c_int,
    pub component_index: ::core::ffi::c_int,
    pub h_samp_factor: ::core::ffi::c_int,
    pub v_samp_factor: ::core::ffi::c_int,
    pub quant_tbl_no: ::core::ffi::c_int,
    pub dc_tbl_no: ::core::ffi::c_int,
    pub ac_tbl_no: ::core::ffi::c_int,
    pub width_in_blocks: JDIMENSION,
    pub height_in_blocks: JDIMENSION,
    pub DCT_h_scaled_size: ::core::ffi::c_int,
    pub DCT_v_scaled_size: ::core::ffi::c_int,
    pub downsampled_width: JDIMENSION,
    pub downsampled_height: JDIMENSION,
    pub component_needed: boolean,
    pub MCU_width: ::core::ffi::c_int,
    pub MCU_height: ::core::ffi::c_int,
    pub MCU_blocks: ::core::ffi::c_int,
    pub MCU_sample_width: ::core::ffi::c_int,
    pub last_col_width: ::core::ffi::c_int,
    pub last_row_height: ::core::ffi::c_int,
    pub quant_table: *mut JQUANT_TBL,
    pub dct_table: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_scan_info {
    pub comps_in_scan: ::core::ffi::c_int,
    pub component_index: [::core::ffi::c_int; 4],
    pub Ss: ::core::ffi::c_int,
    pub Se: ::core::ffi::c_int,
    pub Ah: ::core::ffi::c_int,
    pub Al: ::core::ffi::c_int,
}
pub type J_COLOR_SPACE = ::core::ffi::c_uint;
pub const JCS_RGB565: J_COLOR_SPACE = 16;
pub const JCS_EXT_ARGB: J_COLOR_SPACE = 15;
pub const JCS_EXT_ABGR: J_COLOR_SPACE = 14;
pub const JCS_EXT_BGRA: J_COLOR_SPACE = 13;
pub const JCS_EXT_RGBA: J_COLOR_SPACE = 12;
pub const JCS_EXT_XRGB: J_COLOR_SPACE = 11;
pub const JCS_EXT_XBGR: J_COLOR_SPACE = 10;
pub const JCS_EXT_BGRX: J_COLOR_SPACE = 9;
pub const JCS_EXT_BGR: J_COLOR_SPACE = 8;
pub const JCS_EXT_RGBX: J_COLOR_SPACE = 7;
pub const JCS_EXT_RGB: J_COLOR_SPACE = 6;
pub const JCS_YCCK: J_COLOR_SPACE = 5;
pub const JCS_CMYK: J_COLOR_SPACE = 4;
pub const JCS_YCbCr: J_COLOR_SPACE = 3;
pub const JCS_RGB: J_COLOR_SPACE = 2;
pub const JCS_GRAYSCALE: J_COLOR_SPACE = 1;
pub const JCS_UNKNOWN: J_COLOR_SPACE = 0;
pub type J_DCT_METHOD = ::core::ffi::c_uint;
pub const JDCT_FLOAT: J_DCT_METHOD = 2;
pub const JDCT_IFAST: J_DCT_METHOD = 1;
pub const JDCT_ISLOW: J_DCT_METHOD = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_common_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::core::ffi::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_progress_mgr {
    pub progress_monitor: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub pass_counter: ::core::ffi::c_long,
    pub pass_limit: ::core::ffi::c_long,
    pub completed_passes: ::core::ffi::c_int,
    pub total_passes: ::core::ffi::c_int,
}
pub type j_common_ptr = *mut jpeg_common_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_memory_mgr {
    pub alloc_small: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_large: Option<
        unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub alloc_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JSAMPARRAY,
    >,
    pub alloc_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            JDIMENSION,
            JDIMENSION,
        ) -> JBLOCKARRAY,
    >,
    pub request_virt_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_sarray_ptr,
    >,
    pub request_virt_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            ::core::ffi::c_int,
            boolean,
            JDIMENSION,
            JDIMENSION,
            JDIMENSION,
        ) -> jvirt_barray_ptr,
    >,
    pub realize_virt_arrays: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub access_virt_sarray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_sarray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JSAMPARRAY,
    >,
    pub access_virt_barray: Option<
        unsafe extern "C" fn(
            j_common_ptr,
            jvirt_barray_ptr,
            JDIMENSION,
            JDIMENSION,
            boolean,
        ) -> JBLOCKARRAY,
    >,
    pub free_pool: Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>,
    pub self_destruct: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub max_memory_to_use: ::core::ffi::c_long,
    pub max_alloc_chunk: ::core::ffi::c_long,
}
pub type jvirt_barray_ptr = *mut jvirt_barray_control;
pub type jvirt_sarray_ptr = *mut jvirt_sarray_control;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_error_mgr {
    pub error_exit: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub emit_message: Option<unsafe extern "C" fn(j_common_ptr, ::core::ffi::c_int) -> ()>,
    pub output_message: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub format_message: Option<unsafe extern "C" fn(j_common_ptr, *mut ::core::ffi::c_char) -> ()>,
    pub reset_error_mgr: Option<unsafe extern "C" fn(j_common_ptr) -> ()>,
    pub msg_code: ::core::ffi::c_int,
    pub msg_parm: C2RustUnnamed,
    pub trace_level: ::core::ffi::c_int,
    pub num_warnings: ::core::ffi::c_long,
    pub jpeg_message_table: *const *const ::core::ffi::c_char,
    pub last_jpeg_message: ::core::ffi::c_int,
    pub addon_message_table: *const *const ::core::ffi::c_char,
    pub first_addon_message: ::core::ffi::c_int,
    pub last_addon_message: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i: [::core::ffi::c_int; 8],
    pub s: [::core::ffi::c_char; 80],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_compress_struct {
    pub err: *mut jpeg_error_mgr,
    pub mem: *mut jpeg_memory_mgr,
    pub progress: *mut jpeg_progress_mgr,
    pub client_data: *mut ::core::ffi::c_void,
    pub is_decompressor: boolean,
    pub global_state: ::core::ffi::c_int,
    pub dest: *mut jpeg_destination_mgr,
    pub image_width: JDIMENSION,
    pub image_height: JDIMENSION,
    pub input_components: ::core::ffi::c_int,
    pub in_color_space: J_COLOR_SPACE,
    pub input_gamma: ::core::ffi::c_double,
    pub scale_num: ::core::ffi::c_uint,
    pub scale_denom: ::core::ffi::c_uint,
    pub jpeg_width: JDIMENSION,
    pub jpeg_height: JDIMENSION,
    pub data_precision: ::core::ffi::c_int,
    pub num_components: ::core::ffi::c_int,
    pub jpeg_color_space: J_COLOR_SPACE,
    pub comp_info: *mut jpeg_component_info,
    pub quant_tbl_ptrs: [*mut JQUANT_TBL; 4],
    pub q_scale_factor: [::core::ffi::c_int; 4],
    pub dc_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub ac_huff_tbl_ptrs: [*mut JHUFF_TBL; 4],
    pub arith_dc_L: [UINT8; 16],
    pub arith_dc_U: [UINT8; 16],
    pub arith_ac_K: [UINT8; 16],
    pub num_scans: ::core::ffi::c_int,
    pub scan_info: *const jpeg_scan_info,
    pub raw_data_in: boolean,
    pub arith_code: boolean,
    pub optimize_coding: boolean,
    pub CCIR601_sampling: boolean,
    pub do_fancy_downsampling: boolean,
    pub smoothing_factor: ::core::ffi::c_int,
    pub dct_method: J_DCT_METHOD,
    pub restart_interval: ::core::ffi::c_uint,
    pub restart_in_rows: ::core::ffi::c_int,
    pub write_JFIF_header: boolean,
    pub JFIF_major_version: UINT8,
    pub JFIF_minor_version: UINT8,
    pub density_unit: UINT8,
    pub X_density: UINT16,
    pub Y_density: UINT16,
    pub write_Adobe_marker: boolean,
    pub next_scanline: JDIMENSION,
    pub progressive_mode: boolean,
    pub max_h_samp_factor: ::core::ffi::c_int,
    pub max_v_samp_factor: ::core::ffi::c_int,
    pub min_DCT_h_scaled_size: ::core::ffi::c_int,
    pub min_DCT_v_scaled_size: ::core::ffi::c_int,
    pub total_iMCU_rows: JDIMENSION,
    pub comps_in_scan: ::core::ffi::c_int,
    pub cur_comp_info: [*mut jpeg_component_info; 4],
    pub MCUs_per_row: JDIMENSION,
    pub MCU_rows_in_scan: JDIMENSION,
    pub blocks_in_MCU: ::core::ffi::c_int,
    pub MCU_membership: [::core::ffi::c_int; 10],
    pub Ss: ::core::ffi::c_int,
    pub Se: ::core::ffi::c_int,
    pub Ah: ::core::ffi::c_int,
    pub Al: ::core::ffi::c_int,
    pub block_size: ::core::ffi::c_int,
    pub natural_order: *const ::core::ffi::c_int,
    pub lim_Se: ::core::ffi::c_int,
    pub master: *mut jpeg_comp_master,
    pub main: *mut jpeg_c_main_controller,
    pub prep: *mut jpeg_c_prep_controller,
    pub coef: *mut jpeg_c_coef_controller,
    pub marker: *mut jpeg_marker_writer,
    pub cconvert: *mut jpeg_color_converter,
    pub downsample: *mut jpeg_downsampler,
    pub fdct: *mut jpeg_forward_dct,
    pub entropy: *mut jpeg_entropy_encoder,
    pub script_space: *mut jpeg_scan_info,
    pub script_space_size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct jpeg_destination_mgr {
    pub next_output_byte: *mut JOCTET,
    pub free_in_buffer: size_t,
    pub init_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
    pub empty_output_buffer: Option<unsafe extern "C" fn(j_compress_ptr) -> boolean>,
    pub term_destination: Option<unsafe extern "C" fn(j_compress_ptr) -> ()>,
}
pub type j_compress_ptr = *mut jpeg_compress_struct;
pub const _ISdigit: C2RustUnnamed_0 = 2048;
pub const _ISspace: C2RustUnnamed_0 = 8192;
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed_0 = 8;
pub const _ISpunct: C2RustUnnamed_0 = 4;
pub const _IScntrl: C2RustUnnamed_0 = 2;
pub const _ISblank: C2RustUnnamed_0 = 1;
pub const _ISgraph: C2RustUnnamed_0 = 32768;
pub const _ISprint: C2RustUnnamed_0 = 16384;
pub const _ISxdigit: C2RustUnnamed_0 = 4096;
pub const _ISalpha: C2RustUnnamed_0 = 1024;
pub const _ISlower: C2RustUnnamed_0 = 512;
pub const _ISupper: C2RustUnnamed_0 = 256;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const MAX_COMPONENTS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DCTSIZE2: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NUM_QUANT_TBLS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_COMPS_IN_SCAN: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const JPOOL_IMAGE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
unsafe extern "C" fn text_getc(mut file: *mut FILE) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    ch = getc(file);
    if ch == '#' as i32 {
        loop {
            ch = getc(file);
            if !(ch != '\n' as i32 && ch != EOF) {
                break;
            }
        }
    }
    return ch;
}
unsafe extern "C" fn read_text_integer(
    mut file: *mut FILE,
    mut result: *mut ::core::ffi::c_long,
    mut termchar: *mut ::core::ffi::c_int,
) -> boolean {
    let mut ch: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_long = 0;
    loop {
        ch = text_getc(file);
        if ch == EOF {
            *termchar = ch;
            return FALSE;
        }
        if !(*(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0)
        {
            break;
        }
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        == 0
    {
        *termchar = ch;
        return FALSE;
    }
    val = (ch - '0' as i32) as ::core::ffi::c_long;
    loop {
        ch = text_getc(file);
        if !(ch != EOF) {
            break;
        }
        if *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            break;
        }
        val *= 10 as ::core::ffi::c_long;
        val += (ch - '0' as i32) as ::core::ffi::c_long;
    }
    *result = val;
    *termchar = ch;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn read_quant_tables(
    mut cinfo: j_compress_ptr,
    mut filename: *mut ::core::ffi::c_char,
    mut force_baseline: boolean,
) -> boolean {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut tblno: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut termchar: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_long = 0;
    let mut table: [::core::ffi::c_uint; 64] = [0; 64];
    fp = fopen(filename, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"Can't open table file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return FALSE;
    }
    tblno = 0 as ::core::ffi::c_int;
    while read_text_integer(fp, &raw mut val, &raw mut termchar) != 0 {
        if tblno >= NUM_QUANT_TBLS {
            fprintf(
                stderr,
                b"Too many tables in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
            fclose(fp);
            return FALSE;
        }
        table[0 as ::core::ffi::c_int as usize] = val as ::core::ffi::c_uint;
        i = 1 as ::core::ffi::c_int;
        while i < DCTSIZE2 {
            if read_text_integer(fp, &raw mut val, &raw mut termchar) == 0 {
                fprintf(
                    stderr,
                    b"Invalid table data in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    filename,
                );
                fclose(fp);
                return FALSE;
            }
            table[i as usize] = val as ::core::ffi::c_uint;
            i += 1;
        }
        jpeg_add_quant_table(
            cinfo,
            tblno,
            &raw mut table as *mut ::core::ffi::c_uint,
            (*cinfo).q_scale_factor[tblno as usize],
            force_baseline,
        );
        tblno += 1;
    }
    if termchar != EOF {
        fprintf(
            stderr,
            b"Non-numeric data in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        fclose(fp);
        return FALSE;
    }
    fclose(fp);
    return TRUE;
}
unsafe extern "C" fn read_scan_integer(
    mut file: *mut FILE,
    mut result: *mut ::core::ffi::c_long,
    mut termchar: *mut ::core::ffi::c_int,
) -> boolean {
    let mut ch: ::core::ffi::c_int = 0;
    if read_text_integer(file, result, termchar) == 0 {
        return FALSE;
    }
    ch = *termchar;
    while ch != EOF
        && *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
            & _ISspace as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
    {
        ch = text_getc(file);
    }
    if *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
        & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
        != 0
    {
        if ungetc(ch, file) == EOF {
            return FALSE;
        }
        ch = ' ' as i32;
    } else if ch != EOF && ch != ';' as i32 && ch != ':' as i32 {
        ch = ' ' as i32;
    }
    *termchar = ch;
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn read_scan_script(
    mut cinfo: j_compress_ptr,
    mut filename: *mut ::core::ffi::c_char,
) -> boolean {
    let mut current_block: u64;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut scanno: ::core::ffi::c_int = 0;
    let mut ncomps: ::core::ffi::c_int = 0;
    let mut termchar: ::core::ffi::c_int = 0;
    let mut val: ::core::ffi::c_long = 0;
    let mut scanptr: *mut jpeg_scan_info = ::core::ptr::null_mut::<jpeg_scan_info>();
    let mut scans: [jpeg_scan_info; 100] = [jpeg_scan_info {
        comps_in_scan: 0,
        component_index: [0; 4],
        Ss: 0,
        Se: 0,
        Ah: 0,
        Al: 0,
    }; 100];
    fp = fopen(filename, b"r\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"Can't open scan definition file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return FALSE;
    }
    scanptr = &raw mut scans as *mut jpeg_scan_info;
    scanno = 0 as ::core::ffi::c_int;
    while read_scan_integer(fp, &raw mut val, &raw mut termchar) != 0 {
        if scanno >= MAX_SCANS {
            fprintf(
                stderr,
                b"Too many scans defined in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                filename,
            );
            fclose(fp);
            return FALSE;
        }
        (*scanptr).component_index[0 as ::core::ffi::c_int as usize] = val as ::core::ffi::c_int;
        ncomps = 1 as ::core::ffi::c_int;
        loop {
            if !(termchar == ' ' as i32) {
                current_block = 8457315219000651999;
                break;
            }
            if ncomps >= MAX_COMPS_IN_SCAN {
                fprintf(
                    stderr,
                    b"Too many components in one scan in file %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    filename,
                );
                fclose(fp);
                return FALSE;
            }
            if read_scan_integer(fp, &raw mut val, &raw mut termchar) == 0 {
                current_block = 16699112218191672874;
                break;
            }
            (*scanptr).component_index[ncomps as usize] = val as ::core::ffi::c_int;
            ncomps += 1;
        }
        match current_block {
            8457315219000651999 => {
                (*scanptr).comps_in_scan = ncomps;
                if termchar == ':' as i32 {
                    if read_scan_integer(fp, &raw mut val, &raw mut termchar) == 0
                        || termchar != ' ' as i32
                    {
                        current_block = 16699112218191672874;
                    } else {
                        (*scanptr).Ss = val as ::core::ffi::c_int;
                        if read_scan_integer(fp, &raw mut val, &raw mut termchar) == 0
                            || termchar != ' ' as i32
                        {
                            current_block = 16699112218191672874;
                        } else {
                            (*scanptr).Se = val as ::core::ffi::c_int;
                            if read_scan_integer(fp, &raw mut val, &raw mut termchar) == 0
                                || termchar != ' ' as i32
                            {
                                current_block = 16699112218191672874;
                            } else {
                                (*scanptr).Ah = val as ::core::ffi::c_int;
                                if read_scan_integer(fp, &raw mut val, &raw mut termchar) == 0 {
                                    current_block = 16699112218191672874;
                                } else {
                                    (*scanptr).Al = val as ::core::ffi::c_int;
                                    current_block = 4488286894823169796;
                                }
                            }
                        }
                    }
                } else {
                    (*scanptr).Ss = 0 as ::core::ffi::c_int;
                    (*scanptr).Se = DCTSIZE2 - 1 as ::core::ffi::c_int;
                    (*scanptr).Ah = 0 as ::core::ffi::c_int;
                    (*scanptr).Al = 0 as ::core::ffi::c_int;
                    current_block = 4488286894823169796;
                }
                match current_block {
                    16699112218191672874 => {}
                    _ => {
                        if !(termchar != ';' as i32 && termchar != EOF) {
                            scanptr = scanptr.offset(1);
                            scanno += 1;
                            continue;
                        }
                    }
                }
            }
            _ => {}
        }
        fprintf(
            stderr,
            b"Invalid scan entry format in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        fclose(fp);
        return FALSE;
    }
    if termchar != EOF {
        fprintf(
            stderr,
            b"Non-numeric data in file %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        fclose(fp);
        return FALSE;
    }
    if scanno > 0 as ::core::ffi::c_int {
        scanptr = Some(
            (*(*cinfo).mem)
                .alloc_small
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            cinfo as j_common_ptr,
            JPOOL_IMAGE,
            (scanno as size_t).wrapping_mul(::core::mem::size_of::<jpeg_scan_info>() as size_t),
        ) as *mut jpeg_scan_info;
        memcpy(
            scanptr as *mut ::core::ffi::c_void,
            &raw mut scans as *mut jpeg_scan_info as *const ::core::ffi::c_void,
            (scanno as size_t).wrapping_mul(::core::mem::size_of::<jpeg_scan_info>() as size_t),
        );
        (*cinfo).scan_info = scanptr;
        (*cinfo).num_scans = scanno;
    }
    fclose(fp);
    return TRUE;
}
pub const MAX_SCANS: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn set_quality_ratings(
    mut cinfo: j_compress_ptr,
    mut arg: *mut ::core::ffi::c_char,
    mut force_baseline: boolean,
) -> boolean {
    let mut val: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
    let mut tblno: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_char = 0;
    tblno = 0 as ::core::ffi::c_int;
    while tblno < NUM_QUANT_TBLS {
        if *arg != 0 {
            ch = ',' as i32 as ::core::ffi::c_char;
            if sscanf(
                arg,
                b"%d%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut val,
                &raw mut ch,
            ) < 1 as ::core::ffi::c_int
            {
                return FALSE;
            }
            if ch as ::core::ffi::c_int != ',' as i32 {
                return FALSE;
            }
            (*cinfo).q_scale_factor[tblno as usize] = jpeg_quality_scaling(val);
            while *arg as ::core::ffi::c_int != 0 && {
                let fresh0 = arg;
                arg = arg.offset(1);
                *fresh0 as ::core::ffi::c_int != ',' as i32
            } {}
        } else {
            (*cinfo).q_scale_factor[tblno as usize] = jpeg_quality_scaling(val);
        }
        tblno += 1;
    }
    jpeg_default_qtables(cinfo, force_baseline);
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn set_quant_slots(
    mut cinfo: j_compress_ptr,
    mut arg: *mut ::core::ffi::c_char,
) -> boolean {
    let mut val: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ci: ::core::ffi::c_int = 0;
    let mut ch: ::core::ffi::c_char = 0;
    ci = 0 as ::core::ffi::c_int;
    while ci < MAX_COMPONENTS {
        if *arg != 0 {
            ch = ',' as i32 as ::core::ffi::c_char;
            if sscanf(
                arg,
                b"%d%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut val,
                &raw mut ch,
            ) < 1 as ::core::ffi::c_int
            {
                return FALSE;
            }
            if ch as ::core::ffi::c_int != ',' as i32 {
                return FALSE;
            }
            if val < 0 as ::core::ffi::c_int || val >= NUM_QUANT_TBLS {
                fprintf(
                    stderr,
                    b"JPEG quantization tables are numbered 0..%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    NUM_QUANT_TBLS - 1 as ::core::ffi::c_int,
                );
                return FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val;
            while *arg as ::core::ffi::c_int != 0 && {
                let fresh1 = arg;
                arg = arg.offset(1);
                *fresh1 as ::core::ffi::c_int != ',' as i32
            } {}
        } else {
            (*(*cinfo).comp_info.offset(ci as isize)).quant_tbl_no = val;
        }
        ci += 1;
    }
    return TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn set_sample_factors(
    mut cinfo: j_compress_ptr,
    mut arg: *mut ::core::ffi::c_char,
) -> boolean {
    let mut ci: ::core::ffi::c_int = 0;
    let mut val1: ::core::ffi::c_int = 0;
    let mut val2: ::core::ffi::c_int = 0;
    let mut ch1: ::core::ffi::c_char = 0;
    let mut ch2: ::core::ffi::c_char = 0;
    ci = 0 as ::core::ffi::c_int;
    while ci < MAX_COMPONENTS {
        if *arg != 0 {
            ch2 = ',' as i32 as ::core::ffi::c_char;
            if sscanf(
                arg,
                b"%d%c%d%c\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut val1,
                &raw mut ch1,
                &raw mut val2,
                &raw mut ch2,
            ) < 3 as ::core::ffi::c_int
            {
                return FALSE;
            }
            if ch1 as ::core::ffi::c_int != 'x' as i32 && ch1 as ::core::ffi::c_int != 'X' as i32
                || ch2 as ::core::ffi::c_int != ',' as i32
            {
                return FALSE;
            }
            if val1 <= 0 as ::core::ffi::c_int
                || val1 > 4 as ::core::ffi::c_int
                || val2 <= 0 as ::core::ffi::c_int
                || val2 > 4 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"JPEG sampling factors must be 1..4\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return FALSE;
            }
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = val1;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = val2;
            while *arg as ::core::ffi::c_int != 0 && {
                let fresh2 = arg;
                arg = arg.offset(1);
                *fresh2 as ::core::ffi::c_int != ',' as i32
            } {}
        } else {
            (*(*cinfo).comp_info.offset(ci as isize)).h_samp_factor = 1 as ::core::ffi::c_int;
            (*(*cinfo).comp_info.offset(ci as isize)).v_samp_factor = 1 as ::core::ffi::c_int;
        }
        ci += 1;
    }
    return TRUE;
}
