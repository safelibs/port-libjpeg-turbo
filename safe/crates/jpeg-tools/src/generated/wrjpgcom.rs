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
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
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
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub const READ_BINARY: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"rb\0") };
pub const MAX_COM_LENGTH: ::core::ffi::c_long = 65000 as ::core::ffi::c_long;
static mut infile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
static mut outfile: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
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
unsafe extern "C" fn write_1_byte(mut c: ::core::ffi::c_int) {
    putc(c, outfile);
}
unsafe extern "C" fn write_2_bytes(mut val: ::core::ffi::c_uint) {
    putc(
        (val >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int,
        outfile,
    );
    putc(
        (val & 0xff as ::core::ffi::c_uint) as ::core::ffi::c_int,
        outfile,
    );
}
unsafe extern "C" fn write_marker(mut marker: ::core::ffi::c_int) {
    putc(0xff as ::core::ffi::c_int, outfile);
    putc(marker, outfile);
}
unsafe extern "C" fn copy_rest_of_file() {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getc(infile);
        if !(c != EOF) {
            break;
        }
        putc(c, outfile);
    }
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
unsafe extern "C" fn copy_variable() {
    let mut length: ::core::ffi::c_uint = 0;
    length = read_2_bytes();
    write_2_bytes(length);
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
        write_1_byte(read_1_byte());
        length = length.wrapping_sub(1);
    }
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
unsafe extern "C" fn scan_JPEG_header(mut keep_COM: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut marker: ::core::ffi::c_int = 0;
    if first_marker() != M_SOI {
        fprintf(
            stderr,
            b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
            b"Expected SOI marker first\0" as *const u8 as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    write_marker(M_SOI);
    loop {
        marker = next_marker();
        's_91: {
            let mut current_block_14: u64;
            match marker {
                M_SOF0 => {
                    current_block_14 = 820927060201197550;
                }
                M_SOF1 => {
                    current_block_14 = 820927060201197550;
                }
                M_SOF2 => {
                    current_block_14 = 13580586057089936640;
                }
                M_SOF3 => {
                    current_block_14 = 4639868048223396853;
                }
                M_SOF5 => {
                    current_block_14 = 11611054839759236981;
                }
                M_SOF6 => {
                    current_block_14 = 3925063879267423178;
                }
                M_SOF7 => {
                    current_block_14 = 4382225328929542176;
                }
                M_SOF9 => {
                    current_block_14 = 4011545316205729029;
                }
                M_SOF10 => {
                    current_block_14 = 1458514091039381067;
                }
                M_SOF11 => {
                    current_block_14 = 18406451865117419732;
                }
                M_SOF13 => {
                    current_block_14 = 14207187609806016053;
                }
                M_SOF14 => {
                    current_block_14 = 5914393182858476077;
                }
                M_SOF15 => {
                    current_block_14 = 17918577814250558734;
                }
                M_SOS => {
                    fprintf(
                        stderr,
                        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        b"SOS without prior SOFn\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    exit(EXIT_FAILURE);
                    current_block_14 = 8457315219000651999;
                }
                M_EOI => return marker,
                M_COM => {
                    if keep_COM != 0 {
                        write_marker(marker);
                        copy_variable();
                    } else {
                        skip_variable();
                    }
                    current_block_14 = 8457315219000651999;
                }
                _ => {
                    write_marker(marker);
                    copy_variable();
                    current_block_14 = 8457315219000651999;
                }
            }
            match current_block_14 {
                820927060201197550 => {
                    current_block_14 = 13580586057089936640;
                }
                8457315219000651999 => {
                    break 's_91;
                }
                _ => {}
            }
            match current_block_14 {
                13580586057089936640 => {
                    current_block_14 = 4639868048223396853;
                }
                _ => {}
            }
            match current_block_14 {
                4639868048223396853 => {
                    current_block_14 = 11611054839759236981;
                }
                _ => {}
            }
            match current_block_14 {
                11611054839759236981 => {
                    current_block_14 = 3925063879267423178;
                }
                _ => {}
            }
            match current_block_14 {
                3925063879267423178 => {
                    current_block_14 = 4382225328929542176;
                }
                _ => {}
            }
            match current_block_14 {
                4382225328929542176 => {
                    current_block_14 = 4011545316205729029;
                }
                _ => {}
            }
            match current_block_14 {
                4011545316205729029 => {
                    current_block_14 = 1458514091039381067;
                }
                _ => {}
            }
            match current_block_14 {
                1458514091039381067 => {
                    current_block_14 = 18406451865117419732;
                }
                _ => {}
            }
            match current_block_14 {
                18406451865117419732 => {
                    current_block_14 = 14207187609806016053;
                }
                _ => {}
            }
            match current_block_14 {
                14207187609806016053 => {
                    current_block_14 = 5914393182858476077;
                }
                _ => {}
            }
            match current_block_14 {
                5914393182858476077 => {}
                _ => {}
            }
            return marker;
        }
    }
}
static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"wrjpgcom inserts a textual comment in a JPEG file.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"You can add to or replace any existing comment(s).\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Usage: %s [switches] \0" as *const u8 as *const ::core::ffi::c_char,
        progname,
    );
    fprintf(
        stderr,
        b"[inputfile]\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Switches (names may be abbreviated):\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -replace         Delete any existing comments\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -comment \"text\"  Insert comment with given text\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"  -cfile name      Read comment from named file\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"Notice that you must put quotes around the comment text\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"when you use -comment.\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"If you do not give either -comment or -cfile on the command line,\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"then the comment text is read from standard input.\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"It can be multiple lines, up to %u characters total.\n\0" as *const u8
            as *const ::core::ffi::c_char,
        MAX_COM_LENGTH as ::core::ffi::c_uint,
    );
    fprintf(
        stderr,
        b"You must specify an input JPEG file name when supplying\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stderr,
        b"comment text from standard input.\n\0" as *const u8 as *const ::core::ffi::c_char,
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
    let mut keep_COM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut comment_arg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut comment_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut comment_length: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut marker: ::core::ffi::c_int = 0;
    progname = *argv.offset(0 as ::core::ffi::c_int as isize);
    if progname.is_null()
        || *progname.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        progname = b"wrjpgcom\0" as *const u8 as *const ::core::ffi::c_char;
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
            b"replace\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            keep_COM = 0 as ::core::ffi::c_int;
        } else if keymatch(
            arg,
            b"cfile\0" as *const u8 as *const ::core::ffi::c_char,
            2 as ::core::ffi::c_int,
        ) != 0
        {
            argn += 1;
            if argn >= argc {
                usage();
            }
            comment_file = fopen(
                *argv.offset(argn as isize),
                b"r\0" as *const u8 as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if comment_file.is_null() {
                fprintf(
                    stderr,
                    b"%s: can't open %s\n\0" as *const u8 as *const ::core::ffi::c_char,
                    progname,
                    *argv.offset(argn as isize),
                );
                exit(EXIT_FAILURE);
            }
        } else if keymatch(
            arg,
            b"comment\0" as *const u8 as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) != 0
        {
            argn += 1;
            if argn >= argc {
                usage();
            }
            comment_arg = *argv.offset(argn as isize);
            if *comment_arg.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                == '"' as i32
            {
                comment_arg = malloc(MAX_COM_LENGTH as size_t) as *mut ::core::ffi::c_char;
                if comment_arg.is_null() {
                    fprintf(
                        stderr,
                        b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                        b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    exit(EXIT_FAILURE);
                }
                if strlen(*argv.offset(argn as isize)).wrapping_add(2 as size_t)
                    >= MAX_COM_LENGTH as size_t
                {
                    fprintf(
                        stderr,
                        b"Comment text may not exceed %u bytes\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        MAX_COM_LENGTH as ::core::ffi::c_uint,
                    );
                    exit(EXIT_FAILURE);
                }
                strcpy(
                    comment_arg,
                    (*argv.offset(argn as isize)).offset(1 as ::core::ffi::c_int as isize),
                );
                loop {
                    comment_length = strlen(comment_arg) as ::core::ffi::c_uint;
                    if comment_length > 0 as ::core::ffi::c_uint
                        && *comment_arg
                            .offset(comment_length.wrapping_sub(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int
                            == '"' as i32
                    {
                        *comment_arg.offset(
                            comment_length.wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                        ) = '\0' as i32 as ::core::ffi::c_char;
                        break;
                    } else {
                        argn += 1;
                        if argn >= argc {
                            fprintf(
                                stderr,
                                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                                b"Missing ending quote mark\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            );
                            exit(EXIT_FAILURE);
                        }
                        if strlen(comment_arg)
                            .wrapping_add(strlen(*argv.offset(argn as isize)))
                            .wrapping_add(2 as size_t)
                            >= MAX_COM_LENGTH as size_t
                        {
                            fprintf(
                                stderr,
                                b"Comment text may not exceed %u bytes\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                MAX_COM_LENGTH as ::core::ffi::c_uint,
                            );
                            exit(EXIT_FAILURE);
                        }
                        strcat(
                            comment_arg,
                            b" \0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        strcat(comment_arg, *argv.offset(argn as isize));
                    }
                }
            } else if strlen(*argv.offset(argn as isize)) >= MAX_COM_LENGTH as size_t {
                fprintf(
                    stderr,
                    b"Comment text may not exceed %u bytes\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    MAX_COM_LENGTH as ::core::ffi::c_uint,
                );
                exit(EXIT_FAILURE);
            }
            comment_length = strlen(comment_arg) as ::core::ffi::c_uint;
        } else {
            usage();
        }
        argn += 1;
    }
    if !comment_arg.is_null() && !comment_file.is_null() {
        usage();
    }
    if comment_arg.is_null() && comment_file.is_null() && argn >= argc {
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
    if argn < argc - 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: only one input file\n\0" as *const u8 as *const ::core::ffi::c_char,
            progname,
        );
        usage();
    }
    outfile = stdout;
    if comment_arg.is_null() {
        let mut src_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut c: ::core::ffi::c_int = 0;
        comment_arg = malloc(MAX_COM_LENGTH as size_t) as *mut ::core::ffi::c_char;
        if comment_arg.is_null() {
            fprintf(
                stderr,
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                b"Insufficient memory\0" as *const u8 as *const ::core::ffi::c_char,
            );
            exit(EXIT_FAILURE);
        }
        comment_length = 0 as ::core::ffi::c_uint;
        src_file = if !comment_file.is_null() {
            comment_file
        } else {
            stdin
        };
        loop {
            c = getc(src_file);
            if !(c != EOF) {
                break;
            }
            if comment_length >= MAX_COM_LENGTH as ::core::ffi::c_uint {
                fprintf(
                    stderr,
                    b"Comment text may not exceed %u bytes\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    MAX_COM_LENGTH as ::core::ffi::c_uint,
                );
                exit(EXIT_FAILURE);
            }
            let fresh2 = comment_length;
            comment_length = comment_length.wrapping_add(1);
            *comment_arg.offset(fresh2 as isize) = c as ::core::ffi::c_char;
        }
        if !comment_file.is_null() {
            fclose(comment_file);
        }
    }
    marker = scan_JPEG_header(keep_COM);
    if comment_length > 0 as ::core::ffi::c_uint {
        write_marker(M_COM);
        write_2_bytes(comment_length.wrapping_add(2 as ::core::ffi::c_uint));
        while comment_length > 0 as ::core::ffi::c_uint {
            let fresh3 = comment_arg;
            comment_arg = comment_arg.offset(1);
            write_1_byte(*fresh3 as ::core::ffi::c_int);
            comment_length = comment_length.wrapping_sub(1);
        }
    }
    write_marker(marker);
    copy_rest_of_file();
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
