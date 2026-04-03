use ffi_types::j_decompress_ptr;

extern "C" {
    #[link_name = "jinit_marker_reader"]
    fn c_jinit_marker_reader(cinfo: j_decompress_ptr);
}

pub unsafe fn jinit_marker_reader(cinfo: j_decompress_ptr) {
    c_jinit_marker_reader(cinfo)
}
