use core::mem::{size_of, swap};
use super::shared::unlikely;
use super::shared::mem::MemoryRange;

pub trait Hash32 {
    fn hash32(&self) -> u32;
}

pub trait Mappable<K: PartialEq + Hash32 + ?Sized>: Sized {
    fn id(&self) -> &K;
    fn next(&self) -> usize;
    fn next_mut(&mut self) -> &mut usize;
}

pub struct MappingIter<'a, K, V> where K: PartialEq + Hash32 + ?Sized, V: Mappable<K> {
    pos: usize,
    mapping: &'a Mapping<K, V>,
}

pub struct Mapping<K, V> where K: PartialEq + Hash32 + ?Sized, V: Mappable<K> {
    size: usize,
    capacity: usize,
    items: *mut *mut V,
    phantom: core::marker::PhantomData<*mut K>,
}

impl<'a, K, V> Iterator for MappingIter<'a, K, V> 
    where K: PartialEq + Hash32 + ?Sized, V: Mappable<K>
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let item = self.mapping.items.offset(self.pos as isize);
            if self.pos >= self.mapping.capacity {
                None
            } else if !(*item).is_null() {
                Some(& **item)
            } else {
                self.next()
            }
        }
    }
}

impl<K, V> Mapping<K, V> where K: PartialEq + Hash32 + ?Sized, V: Mappable<K> {
    pub fn from(allocator: &mut MemoryRange, capacity: usize) -> Option<Self> {
        let bytes = size_of::<V>() * capacity;
        allocator.alloc_bytes(bytes).and_then(|bytes| Some(Self {
            size: 0,
            capacity: capacity,
            items: bytes as *mut *mut _,
            phantom: core::marker::PhantomData,
        }))
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> MappingIter<'a, K, V> {
        MappingIter {
            pos: 0,
            mapping: self,
        }
    }

    #[inline]
    pub unsafe fn expand(&mut self, maximum: usize) -> bool {
        let capacity = (self.capacity << 1).min(maximum);
        if capacity >= self.capacity {
            self.capacity = capacity;
        }
        capacity >= self.capacity
    }

    pub fn find(&self, key: &K) -> Option<&mut V> {
        unsafe {
            if unlikely(self.size == 0) {
                return None
            } else {
                let mask = self.capacity - 1;
                let start = key.hash32() as usize & mask;
                let mut index = start;

                let mut slot = self.items.offset(index as isize);
                while !(*slot).is_null() && !(**slot).id().eq(key) {
                    index = (index + 1) & mask;
                    if index == start {
                        return None
                    } else {
                        slot = self.items.offset(index as isize);
                    }
                }

                Some(&mut **slot)
            }
        }
    }

    pub fn insert(&mut self, mut item: *mut V) -> Option<()> {
        unsafe {
            if unlikely(self.size == self.capacity) {
                None
            } else {
                self.size += 1;
                let mask = self.capacity - 1;
                let mut index = (*item).id().hash32() as usize & mask;

                for _ in 0..self.capacity {
                    let slot = self.items.offset(index as isize);
                    if (*slot).is_null() {
                        return Some(*slot = item);
                    } else if (**slot).next() < (*item).next() {
                        swap(&mut *slot, &mut item);
                    }
                    *(*item).next_mut() += 1;
                    index = (index + 1) & mask;
                }

                None
            }
        }
    }
}