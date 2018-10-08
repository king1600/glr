#include "classfile.h"                         

uintptr_t glr_class_load(glr_vm_t* vm, uint8_t* bytes, size_t size) {
    glr_reader_t reader;
    reader.pos.u8 = bytes;
    reader.end = bytes + size;

    // check bytecode magic "$GLR"
    static const char* GLR_MAGIC = "$GLR";
    GLR_READ(reader, u32, uint32_t magic, GLR_CLASS_ERR_MAGIC);
    if (magic != *((const uint32_t*) GLR_MAGIC))
        return GLR_CLASS_ERR_MAGIC;

    // check bytecode version
    GLR_READ(reader, u8, uint8_t version, GLR_CLASS_ERR_VERSION);
    if (version < GLR_VERSION)
        return GLR_CLASS_ERR_VERSION;

    // check bytecode access modifiers
    GLR_READ(reader, u8, uint8_t access, GLR_CLASS_ERR_ACCESS):
    if (access == 0)
        return GLR_CLASS_ERR_ACCESS;
    

    return GLR_CLASS_ERROR;
}