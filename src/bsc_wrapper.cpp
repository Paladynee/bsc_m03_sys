#include <stdint.h>

#define main bsc_m03_original_main
#include "../bsc-m03/bsc-m03.cpp"
#undef main

extern "C" {

int32_t libbsc_compress_memory_block_u8(uint8_t * buffer, int32_t block_size)
{
    return compress_memory_block<uint8_t>(buffer, block_size);
}

int32_t libbsc_compress_memory_block_u16(uint8_t * buffer, int32_t block_size)
{
    return compress_memory_block<uint16_t>(buffer, block_size);
}

int32_t libbsc_decompress_memory_block_c(uint8_t * buffer, int32_t compressed_size, int32_t block_size)
{
    return decompress_memory_block(buffer, compressed_size, block_size);
}

} // extern "C"
