pub trait Cheat {
	pub fn get_name<'a>(&'a self) -> &'a str;
	pub fn process_usercmd(&mut self, &mut sdk::CUserCmd);
}