pub mod environment;
pub mod obj;
pub mod multimethod;
pub mod pattern;

pub use self::environment::Environment;
pub use self::multimethod::*;
pub use self::obj::{Obj, ObjKind};
pub use self::pattern::*;