use super::{ConstPool, Mapping, Mappable, Hash32};

#[repr(u8)]
pub enum Class {
    Enum(ClassFile),
    Struct(ClassFile),
    Module(ClassFile),
}

#[repr(u8)]
pub enum Field {
    Module(FieldContext, u16),
    Struct(FieldContext, u16, u16),
    Enum(FieldContext, u16, Option<*mut Field>),
}

pub struct FieldContext {
    class: *mut Class,
    next_field: usize,
}

pub struct Method {
    name: u16,
    pub access: u8,
    next_method: usize,
    pub code: *const u8,
    pub class: *mut Class,
}

pub struct ClassFile {
    pub access: u8,
    pub next_class: usize,
    pub const_pool: ConstPool,
    pub fields: Option<Mapping<str, Field>>,
    pub methods: Option<Mapping<str, Method>>,
}

impl Hash32 for str {
    fn hash32(&self) -> u32 {
        const FNV_PRIME: u32 = 16777619;
        const FNV_OFFSET: u32 = 2166136261;
        self.as_bytes().iter().fold(FNV_OFFSET,
            |hash, &byte| (hash ^ byte as u32) * FNV_PRIME)
    }
}

impl Mappable<str> for Class {
    fn id(&self) -> &str {
        self.class_file().const_pool.get_str(0).unwrap_or("")
    }

    fn next(&self) -> usize {
        self.class_file().next_class
    }

    fn next_mut(&mut self) -> &mut usize {
        &mut self.class_file_mut().next_class
    }
}

impl Mappable<str> for Field {
    fn id(&self) -> &str {
        self.name()
    }

    fn next(&self) -> usize {
        self.context().next_field
    }

    fn next_mut(&mut self) -> &mut usize {
        &mut self.context_mut().next_field
    }
}

impl Mappable<str> for Method {
    fn id(&self) -> &str {
        self.name()
    }

    fn next(&self) -> usize {
        self.next_method
    }

    fn next_mut(&mut self) -> &mut usize {
        &mut self.next_method
    }
}

impl Class {
    #[inline]
    pub fn class_file(&self) -> &ClassFile {
        match self {
            Class::Module(class_file) |
            Class::Struct(class_file) |
            Class::Enum(class_file) => class_file
        }
    }

    #[inline]
    pub fn class_file_mut(&mut self) -> &mut ClassFile {
        match self {
            Class::Module(class_file) |
            Class::Struct(class_file) |
            Class::Enum(class_file) => class_file
        }
    }
}

impl Method {
    #[inline]
    pub fn const_pool(&self) -> &ConstPool {
        unsafe { &(*self.class).class_file().const_pool }
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.const_pool().get_str(self.name as usize).unwrap_or("")
    }
}

impl Field {
    #[inline]
    pub fn const_pool(&self) -> &ConstPool {
        unsafe { &(*self.context().class).class_file().const_pool }
    }

    #[inline]
    pub fn context(&self) -> &FieldContext {
        match self {
            Field::Module(context, _)   |
            Field::Enum(context, _, _)  |
            Field::Struct(context, _, _) => context
        }
    }

    #[inline]
    pub fn context_mut(&mut self) -> &mut FieldContext {
        match self {
            Field::Module(context, _)   |
            Field::Enum(context, _, _)  |
            Field::Struct(context, _, _) => context
        }
    }

    pub fn name(&self) -> &str {
        self.const_pool().get_str(match self {
            Field::Module(_, index)     |
            Field::Enum(_, index, _)    |
            Field::Struct(_, index, _) => *index as usize
        }).unwrap_or("")
    }
}