mod builder;
mod cursor;
mod delta;
mod delta_serde;
mod iterator;

pub use builder::*;
pub use cursor::*;
pub use delta::*;
pub use iterator::*;

pub const NEW_LINE: &'static str = "\n";
pub const WHITESPACE: &'static str = " ";
