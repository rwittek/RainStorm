#![macro_escape]
use core::prelude::*;
use core;
use core::fmt::FormatWriter;
use core::raw::Repr;
use libc;


macro_rules! log(

	// TODO: could make an oopsie if we have several threads
    ($($arg:tt)*) => ({
		#[allow(unused_imports)] // we need this for log_fmt
		use core::fmt::FormatWriter;
		let _ =  format_args!(::logging::log_fmt, $($arg)*).ok().unwrap();
    })
)



macro_rules! quit(
	($($arg:tt)*) => ({
		log!($($arg)*);
		unsafe { ::libc::exit(1) }
    })
)

static mut LOGGER: Option<Logger> = None;

struct Logger {
	fd: libc::c_int
}

impl Logger {
	unsafe fn new(fd: libc::c_int) -> Logger {
		// if we validate fd, we can make this safe probably
		Logger { fd: fd }
	}
}
impl core::fmt::FormatWriter for Logger {
	fn write(&mut self, bytes: &[u8]) -> core::fmt::Result {
		let repr = bytes.repr();
		let written_len = unsafe { libc::write(self.fd, core::mem::transmute(repr.data), repr.len as u32) };
		
		match written_len == (repr.len as i32) {
			// did we write all the data?
			true => Ok(()),
			false => Err(core::fmt::WriteError)
		}
	}
}

pub unsafe fn set_fd(fd: libc::c_int) -> core::result::Result<(), ()> {
	match LOGGER {
		Some(_logger) => Err(()), // already exists!
		None => {
			LOGGER = Some(Logger::new(fd));
			Ok(())
		}
	}
}

pub fn log_fmt(args: &core::fmt::Arguments) -> core::fmt::Result {
	unsafe { match LOGGER {
		Some(mut logger) => {logger.write_fmt(args)},
		None => Err(core::fmt::WriteError)
	}}
}