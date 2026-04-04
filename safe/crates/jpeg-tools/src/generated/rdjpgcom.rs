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
extern "C" {
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn tolower(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
pub type C2RustUnnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LC_CTYPE: ::core::ffi::c_int = __LC_CTYPE;
pub const READ_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
static mut infile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
unsafe extern "C" fn read_1_byte() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    c = getc(infile);
    if c == EOF {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Premature EOF in JPEG file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return c;
}
unsafe extern "C" fn read_2_bytes() -> ::core::ffi::c_uint {
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    c1 = getc(infile);
    if c1 == EOF {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Premature EOF in JPEG file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    c2 = getc(infile);
    if c2 == EOF {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Premature EOF in JPEG file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return ((c1 as ::core::ffi::c_uint) << 8 as ::core::ffi::c_int)
        .wrapping_add(c2 as ::core::ffi::c_uint);
}
pub const M_SOF0: ::core::ffi::c_int = 192;
pub const M_SOF1: ::core::ffi::c_int = 193;
pub const M_SOF2: ::core::ffi::c_int = 194;
pub const M_SOF3: ::core::ffi::c_int = 195;
pub const M_SOF5: ::core::ffi::c_int = 197;
pub const M_SOF6: ::core::ffi::c_int = 198;
pub const M_SOF7: ::core::ffi::c_int = 199;
pub const M_SOF9: ::core::ffi::c_int = 201;
pub const M_SOF10: ::core::ffi::c_int = 202;
pub const M_SOF11: ::core::ffi::c_int = 203;
pub const M_SOF13: ::core::ffi::c_int = 205;
pub const M_SOF14: ::core::ffi::c_int = 206;
pub const M_SOF15: ::core::ffi::c_int = 207;
pub const M_SOI: ::core::ffi::c_int = 0xd8 as ::core::ffi::c_int;
pub const M_EOI: ::core::ffi::c_int = 217;
pub const M_SOS: ::core::ffi::c_int = 218;
pub const M_APP12: ::core::ffi::c_int = 236;
pub const M_COM: ::core::ffi::c_int = 254;
unsafe extern "C" fn next_marker() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    let mut discarded_bytes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    c = read_1_byte();
    while c != 0xff as ::core::ffi::c_int {
        discarded_bytes += 1;
        c = read_1_byte();
    }
    loop {
        c = read_1_byte();
        if !(c == 0xff as ::core::ffi::c_int) {
            break;
        }
    }
    if discarded_bytes != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Warning: garbage data found in JPEG file\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
    }
    return c;
}
unsafe extern "C" fn first_marker() -> ::core::ffi::c_int {
    let mut c1: ::core::ffi::c_int = 0;
    let mut c2: ::core::ffi::c_int = 0;
    c1 = getc(infile);
    c2 = getc(infile);
    if c1 != 0xff as ::core::ffi::c_int || c2 != M_SOI {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Not a JPEG file\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    return c2;
}
unsafe extern "C" fn skip_variable() {
    let mut length: ::core::ffi::c_uint = 0;
    length = read_2_bytes();
    if length < 2 as ::core::ffi::c_uint {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Erroneous JPEG marker length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    length = length.wrapping_sub(2 as ::core::ffi::c_uint);
    while length > 0 as ::core::ffi::c_uint {
        read_1_byte();
        length = length.wrapping_sub(1);
    }
}
unsafe extern "C" fn process_COM(mut raw: ::core::ffi::c_int) {
    let mut length: ::core::ffi::c_uint = 0;
    let mut ch: ::core::ffi::c_int = 0;
    let mut lastch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    setlocale(LC_CTYPE, b"\0" as *const u8 as *const ::core::ffi::c_char);
    length = read_2_bytes();
    if length < 2 as ::core::ffi::c_uint {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Erroneous JPEG marker length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    length = length.wrapping_sub(2 as ::core::ffi::c_uint);
    while length > 0 as ::core::ffi::c_uint {
        ch = read_1_byte();
        if raw != 0 {
            putc(ch, stdout);
        } else if ch == '\r' as i32 {
            printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
        } else if ch == '\n' as i32 {
            if lastch != '\r' as i32 {
                printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
            }
        } else if ch == '\\' as i32 {
            printf(b"\\\\\0" as *const u8 as *const ::core::ffi::c_char);
        } else if *(*__ctype_b_loc()).offset(ch as isize) as ::core::ffi::c_int
            & _ISprint as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            putc(ch, stdout);
        } else {
            printf(
                b"\\%03o\0" as *const u8 as *const ::core::ffi::c_char,
                ch as ::core::ffi::c_uint,
            );
        }
        lastch = ch;
        length = length.wrapping_sub(1);
    }
    printf(b"\n\0" as *const u8 as *const ::core::ffi::c_char);
    setlocale(LC_CTYPE, b"C\0" as *const u8 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn process_SOFn(mut marker: ::core::ffi::c_int) {
    let mut length: ::core::ffi::c_uint = 0;
    let mut image_height: ::core::ffi::c_uint = 0;
    let mut image_width: ::core::ffi::c_uint = 0;
    let mut data_precision: ::core::ffi::c_int = 0;
    let mut num_components: ::core::ffi::c_int = 0;
    let mut process: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ci: ::core::ffi::c_int = 0;
    length = read_2_bytes();
    data_precision = read_1_byte();
    image_height = read_2_bytes();
    image_width = read_2_bytes();
    num_components = read_1_byte();
    match marker {
        M_SOF0 => {
            process = b"Baseline\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF1 => {
            process = b"Extended sequential\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF2 => {
            process = b"Progressive\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF3 => {
            process = b"Lossless\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF5 => {
            process = b"Differential sequential\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF6 => {
            process = b"Differential progressive\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF7 => {
            process = b"Differential lossless\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF9 => {
            process = b"Extended sequential, arithmetic coding\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        M_SOF10 => {
            process =
                b"Progressive, arithmetic coding\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF11 => {
            process = b"Lossless, arithmetic coding\0" as *const u8 as *const ::core::ffi::c_char;
        }
        M_SOF13 => {
            process = b"Differential sequential, arithmetic coding\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        M_SOF14 => {
            process = b"Differential progressive, arithmetic coding\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        M_SOF15 => {
            process = b"Differential lossless, arithmetic coding\0" as *const u8
                as *const ::core::ffi::c_char;
        }
        _ => {
            process = b"Unknown\0" as *const u8 as *const ::core::ffi::c_char;
        }
    }
    printf(
        b"JPEG image is %uw * %uh, %d color components, %d bits per sample\n\0" as *const u8
            as *const ::core::ffi::c_char,
        image_width,
        image_height,
        num_components,
        data_precision,
    );
    printf(
        b"JPEG process: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
        process,
    );
    if length
        != (8 as ::core::ffi::c_int + num_components * 3 as ::core::ffi::c_int)
            as ::core::ffi::c_uint
    {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Bogus SOF marker length\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    ci = 0 as ::core::ffi::c_int;
    while ci < num_components {
        read_1_byte();
        read_1_byte();
        read_1_byte();
        ci += 1;
    }
}
unsafe extern "C" fn scan_JPEG_header(
    mut verbose: ::core::ffi::c_int,
    mut raw: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut marker: ::core::ffi::c_int = 0;
    if first_marker() != M_SOI {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Expected SOI marker first\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    loop {
        marker = next_marker();
        let mut current_block_14: u64;
        match marker {
            M_SOF0 => {
                current_block_14 = 352174004638099311;
            }
            M_SOF1 => {
                current_block_14 = 352174004638099311;
            }
            M_SOF2 => {
                current_block_14 = 8778161631388895614;
            }
            M_SOF3 => {
                current_block_14 = 363603568489877559;
            }
            M_SOF5 => {
                current_block_14 = 17710424497559986530;
            }
            M_SOF6 => {
                current_block_14 = 10113168388784134294;
            }
            M_SOF7 => {
                current_block_14 = 14240390698451773031;
            }
            M_SOF9 => {
                current_block_14 = 11164883624429522243;
            }
            M_SOF10 => {
                current_block_14 = 13611280479612436035;
            }
            M_SOF11 => {
                current_block_14 = 12085232930305121004;
            }
            M_SOF13 => {
                current_block_14 = 16986240651770106891;
            }
            M_SOF14 => {
                current_block_14 = 4601750298003404437;
            }
            M_SOF15 => {
                current_block_14 = 11045451106458149833;
            }
            M_SOS => return marker,
            M_EOI => return marker,
            M_COM => {
                process_COM(raw);
                current_block_14 = 4495394744059808450;
            }
            M_APP12 => {
                if verbose != 0 {
                    printf(b"APP12 contains:\n\0" as *const u8 as *const ::core::ffi::c_char);
                    process_COM(raw);
                } else {
                    skip_variable();
                }
                current_block_14 = 4495394744059808450;
            }
            _ => {
                skip_variable();
                current_block_14 = 4495394744059808450;
            }
        }
        match current_block_14 {
            352174004638099311 => {
                current_block_14 = 8778161631388895614;
            }
            _ => {}
        }
        match current_block_14 {
            8778161631388895614 => {
                current_block_14 = 363603568489877559;
            }
            _ => {}
        }
        match current_block_14 {
            363603568489877559 => {
                current_block_14 = 17710424497559986530;
            }
            _ => {}
        }
        match current_block_14 {
            17710424497559986530 => {
                current_block_14 = 10113168388784134294;
            }
            _ => {}
        }
        match current_block_14 {
            10113168388784134294 => {
                current_block_14 = 14240390698451773031;
            }
            _ => {}
        }
        match current_block_14 {
            14240390698451773031 => {
                current_block_14 = 11164883624429522243;
            }
            _ => {}
        }
        match current_block_14 {
            11164883624429522243 => {
                current_block_14 = 13611280479612436035;
            }
            _ => {}
        }
        match current_block_14 {
            13611280479612436035 => {
                current_block_14 = 12085232930305121004;
            }
            _ => {}
        }
        match current_block_14 {
            12085232930305121004 => {
                current_block_14 = 16986240651770106891;
            }
            _ => {}
        }
        match current_block_14 {
            16986240651770106891 => {
                current_block_14 = 4601750298003404437;
            }
            _ => {}
        }
        match current_block_14 {
            4601750298003404437 => {
                current_block_14 = 11045451106458149833;
            }
            _ => {}
        }
        match current_block_14 {
            11045451106458149833 => {
                if verbose != 0 {
                    process_SOFn(marker);
                } else {
                    skip_variable();
                }
            }
            _ => {}
        }
    }
}
static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"rdjpgcom displays any textual comments in a JPEG file.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Usage: %s [switches] [inputfile]\n\0" as *const u8 as *const ::core::ffi::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"Switches (names may be abbreviated):\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -raw        Display non-printable characters in comments (unsafe)\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -verbose    Also display dimensions of JPEG image\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    exit(EXIT_FAILURE);
}
unsafe extern "C" fn keymatch(
    mut arg: *mut ::core::ffi::c_char,
    mut keyword: *const ::core::ffi::c_char,
    mut minchars: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ca: ::core::ffi::c_int = 0;
    let mut ck: ::core::ffi::c_int = 0;
    let mut nmatched: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let fresh0 = arg;
        arg = arg.offset(1);
        ca = *fresh0 as ::core::ffi::c_int;
        if !(ca != '\0' as i32) {
            break;
        }
        let fresh1 = keyword;
        keyword = keyword.offset(1);
        ck = *fresh1 as ::core::ffi::c_int;
        if ck == '\0' as i32 {
            return 0 as ::core::ffi::c_int;
        }
        if *(*__ctype_b_loc()).offset(ca as isize) as ::core::ffi::c_int
            & _ISupper as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            ca = tolower(ca);
        }
        if ca != ck {
            return 0 as ::core::ffi::c_int;
        }
        nmatched += 1;
    }
    if nmatched < minchars {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut argn: ::core::ffi::c_int = 0;
    let mut arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut verbose: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut raw: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    progname = *argv.offset(0 as ::core::ffi::c_int as isize);
    if progname.is_null()
        || *progname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        progname = b"rdjpgcom\0" as *const u8 as *const ::core::ffi::c_char;
    }
    argn = 1 as ::core::ffi::c_int;
    while argn < argc {
        arg = *argv.offset(argn as isize);
        if *arg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != '-' as i32 {
            break;
        }
        arg = arg.offset(1);
        if keymatch(
            arg,
            b"verbose\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            verbose += 1;
        } else if keymatch(
            arg,
            b"raw\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            raw = 1 as ::core::ffi::c_int;
        } else {
            usage();
        }
        argn += 1;
    }
    if argn < argc - 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: only one input file\n\0" as *const u8 as *const ::core::ffi::c_char,
            progname,
        );
        usage();
    }
    if argn < argc {
        infile = fopen(*argv.offset(argn as isize), READ_BINARY.as_ptr()) as *mut FILE;
        if infile.is_null() {
            fprintf(
                stderr,
                b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                progname,
                *argv.offset(argn as isize),
            );
            exit(EXIT_FAILURE);
        }
    } else {
        infile = stdin;
    }
    scan_JPEG_header(verbose, raw);
    exit(EXIT_SUCCESS);
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
