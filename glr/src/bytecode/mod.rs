#[allow(dead_code)]
pub mod reader;
#[allow(dead_code)]
pub mod loader;
#[allow(dead_code)]
pub mod class_map;
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
pub use self::class_map::*;
pub use self::class_load::*;
pub use self::class_file::*;
pub use self::const_pool::*;
pub use self::interpreter::*;

pub enum ClassError {
    OutOfMemory,
    BadClassType,
    BadClassName,
    BadClassMagic,
    BadFieldSize,
    BadMethodSize,
    BadAccessModifier,
}