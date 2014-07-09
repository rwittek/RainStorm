pub use libc::c_char;
use core::prelude::*;
// opaque phantom types
pub enum Engine {}

#[link(name="wrapper", kind="static")]
extern {
	pub fn getptr_engine() -> Option<&'static mut Engine>;
	pub fn engine_servercmd(engine: & mut Engine, cmd_string: * const c_char, reliable: bool);
}