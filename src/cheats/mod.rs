use sdk;


pub struct CheatManager {
	
pub trait Cheat {
	fn new() -> Self;
	fn get_name<'a>(&'a self) -> &'a str;
	fn process_usercmd(&mut self, &mut sdk::CUserCmd);
}

struct CheatList { // static linked list
	next: Option<&'static mut CheatList>,
	cheat: &'static Cheat
}