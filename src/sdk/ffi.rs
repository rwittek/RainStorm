pub use libc::c_char;

// opaque phantom types
pub enum Engine {}

#[link(name="wrapper", kind="static")]
extern {
	pub fn getptr_engine() -> * mut Engine; // MAYBE NULL
	pub fn engine_servercmd(engine: & mut Engine, cmd_string: * const c_char, reliable: bool);
}