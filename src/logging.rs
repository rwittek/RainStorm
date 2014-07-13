use core::prelude::*;
use core;
use core::fmt::FormatWriter;
use core::raw::Repr;
use libc;

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
		Some(logger) => Err(()), // already exists!
		None => {
			LOGGER = Some(Logger::new(fd));
			Ok(())
		}
	}
}

pub fn log(args: &core::fmt::Arguments) -> core::fmt::Result {
	unsafe { match LOGGER {
		Some(mut logger) => {logger.write_fmt(args); Ok(())},
		None => Err(core::fmt::WriteError)
	}}
}