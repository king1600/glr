use core::str::from_utf8_unchecked;
use core::slice::{from_raw_parts, from_raw_parts_mut};

pub struct ConstPool {
    size: usize,
    pool: *mut Const,
}

#[repr(u8)]
pub enum Const {
    Int(i64),
    UInt(u64),
    Float(f64),
    Str(*const u8, usize),
}

impl ConstPool {
    #[inline]
    pub fn new(pool: *mut Const, size: usize) -> Self {
        Self { size, pool }
    }

    #[inline]
    pub fn as_slice(&self) -> &[Const] {
        unsafe { from_raw_parts(self.pool, self.size) }
    }

    #[inline]
    pub fn as_slice_mut(&mut self) -> &mut [Const] {
        unsafe { from_raw_parts_mut(self.pool, self.size) }
    }

    #[inline]
    pub fn get_int(&self, index: usize) -> Option<i64> {
        match self.as_slice().get(index) {
            Some(&Const::Int(value)) => Some(value),
            _ => None
        }
    }

    #[inline]
    pub fn get_uint(&self, index: usize) -> Option<u64> {
        match self.as_slice().get(index) {
            Some(&Const::UInt(value)) => Some(value),
            _ => None
        }
    }

    #[inline]
    pub fn get_float(&self, index: usize) -> Option<f64> {
        match self.as_slice().get(index) {
            Some(&Const::Float(value)) => Some(value),
            _ => None
        }
    }

    #[inline]
    pub fn get_str(&self, index: usize) -> Option<&str> {
        match self.as_slice().get(index) {
            Some(&Const::Str(text, size)) => Some(unsafe {
                from_utf8_unchecked(from_raw_parts(text, size))
            }),
            _ => None
        }
    }
}