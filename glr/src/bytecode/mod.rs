#[allow(dead_code)]
pub mod reader;
#[allow(dead_code)]
pub mod loader;
#[allow(dead_code)]
pub mod mapping;
#[allow(dead_code)]
pub mod opcodes;
#[allow(dead_code)]
pub mod class_load;
#[allow(dead_code)]
pub mod class_file;
#[allow(dead_code)]
pub mod const_pool;
#[allow(dead_code)]
pub mod interpreter;

pub use super::*;

pub use self::reader::*;
pub use self::loader::*;
pub use self::mapping::*;
pub use self::opcodes::*;
pub use self::class_load::*;
pub use self::class_file::*;
pub use self::const_pool::*;
pub use self::interpreter::*;

pub type ClassResult<T> = Result<T, ClassError>;

pub enum ClassError {
    OutOfMemory,

    BadClassType,
    BadClassName,
    BadClassMagic,
    BadAccessModifier,

    BadCodePos,
    BadCodeSize,
    BadCodeData,

    BadEnumSize,
    BadEnumField,
    BadFieldSize,
    BadMethodSize,

    BadConstSize,
    BadConstType,
    BadConstData,
    BadConstIndex,
}