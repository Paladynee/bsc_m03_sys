use std::ffi::c_int;

unsafe extern "C" {
    pub fn libbsc_compress_memory_block_u8(buffer: *mut u8, block_size: c_int) -> c_int;
    pub fn libbsc_compress_memory_block_u16(buffer: *mut u8, block_size: c_int) -> c_int;
    pub fn libbsc_decompress_memory_block_c(buffer: *mut u8, compressed_size: c_int, block_size: c_int) -> c_int;
}
