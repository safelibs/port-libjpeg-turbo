use ffi_types::j_decompress_ptr;

extern "C" {
    #[link_name = "jinit_huff_decoder"]
    fn c_jinit_huff_decoder(cinfo: j_decompress_ptr);
}

pub unsafe fn jinit_huff_decoder(cinfo: j_decompress_ptr) {
    c_jinit_huff_decoder(cinfo)
}
