use super::ConstPool;

#[repr(u8)]
pub enum Class {
    Module(ClassFile),
    Struct(ClassFile),
    Enum(ClassFile),
}

#[repr(u8)]
pub enum Field {
    Module(u16, Option<*mut Field>),
    Struct(u16, u16, Option<*mut Field>),
    Enum(u16, Option<*mut Field>, Option<*mut Field>),
}

pub struct Method {
    name: u16,
    signature: u16,
    next: Option<*mut Method>,
}

pub struct ClassFile {
    access: u8,
    next_class: usize,
    const_pool: ConstPool,
    methods: Option<*mut Method>,
}

impl Class {
    #[inline]
    pub fn next_class(&self) -> usize {
        self.as_class_file().next_class
    }

    #[inline]
    pub fn bump_next_class(&mut self) {
        self.as_class_file_mut().next_class += 1;
    }

    #[inline]
    pub fn name(&self) -> Option<&str> {
        self.as_class_file().const_pool.get_str(0)
    }

    #[inline]
    pub fn is_called(&self, class_name: &str) -> bool {
        if let Some(this_name) = self.name() {
            this_name == class_name
        } else {
            false
        }
    }

    #[inline]
    fn as_class_file(&self) -> &ClassFile {
        match self {
            Class::Module(class_file) |
            Class::Struct(class_file) |
            Class::Enum(class_file) => class_file
        }
    }

    #[inline]
    fn as_class_file_mut(&mut self) -> &mut ClassFile {
        match self {
            Class::Module(class_file) |
            Class::Struct(class_file) |
            Class::Enum(class_file) => class_file
        }
    }
}