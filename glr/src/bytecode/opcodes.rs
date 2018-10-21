use self::TypeSize::*;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
pub enum TypeSize {
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
}

static TYPE_SIZE: [TypeSize; 8] = [
    U8,
    U16,
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
];

impl TypeSize {
    #[inline]
    pub fn from(value: u8) -> Option<Self> {
        try { TYPE_SIZE.get(value as usize)?.clone() }
    }

    #[inline]
    pub fn extract(opcode: u8) -> (Option<Self>, u8) {
        (Self::from(opcode >> 5), opcode & 0b111)
    }
}