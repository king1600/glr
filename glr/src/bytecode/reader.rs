

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
            let bytes = bytes.as_ptr() as *const _;
            let mut value: T = core::mem::uninitialized();
            core::ptr::copy_nonoverlapping(bytes, &mut value, 1);
            Some(value)
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