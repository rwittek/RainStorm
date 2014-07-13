use sdk;

pub trait Cheat {
	fn get_name<'a>(&'a self) -> &'a str;
	fn process_usercmd(&mut self, &mut sdk::CUserCmd);
}