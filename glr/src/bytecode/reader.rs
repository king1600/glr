

pub struct Reader<'a> {
    pos: usize,
    bytes: &'a [u8]
}

impl<'a> From<&'a [u8]> for Reader<'a> {
    fn from(bytes: &'a [u8]) -> Self {
        Self { pos: 0, bytes }
    }
}

impl<'a> Reader<'a> {
    pub fn read<T: Sized>(&mut self) -> Option<T> {
        self.read_bytes(core::mem::size_of::<T>()).and_then(|bytes| unsafe {
            Some(core::ptr::read_unaligned(bytes.as_ptr() as *const _ ))
        })
    }

    pub fn read_bytes(&mut self, bytes: usize) -> Option<&[u8]> {
        if self.pos + bytes <= self.bytes.len() {
            let pos = self.pos;
            self.pos += bytes;
            Some(&self.bytes[pos..pos + bytes])
        } else {
            None
        }
    }
}