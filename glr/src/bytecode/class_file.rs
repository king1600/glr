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
    pub access: u8,
    pub next_class: usize,
    pub const_pool: ConstPool,
    pub fields: Option<*mut Field>,
    pub methods: Option<*mut Method>,
}

pub trait Nextable {
    fn bump_next(&mut self) {}

    fn set_next(&mut self, next: *mut Self);
    
    fn compare_next(&self, other: &Self) -> bool { true }
}

impl Nextable for Method {
    fn set_next(&mut self, next: *mut Self) {
        self.next = Some(next)
    }
}

impl Nextable for Field {
    fn set_next(&mut self, next: *mut Self) {
        let next_field = match self {
            Field::Enum(_, _, next_field)   |
            Field::Module(_, next_field)    |
            Field::Struct(_, _, next_field) => next_field,
        };
        *next_field = Some(next);
    }
}

impl Nextable for Class {
    fn set_next(&mut self, _next: *mut Self) {}

    fn bump_next(&mut self) {
        self.as_class_file_mut().next_class += 1;
    }
    
    fn compare_next(&self, other: &Self) -> bool {
        self.as_class_file().next_class < other.as_class_file().next_class
    }
}

impl Class {
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