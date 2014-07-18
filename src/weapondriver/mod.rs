pub enum TargetQuality {
	Miss, // we will do no damage
	Poor, // minimal damage, last resort only, should probably wait to shoot
	Good, // a neat amout of damage... "default" for most weapons
	Great // sniper headshots, sniper bodyshots w/ charged OHK, meatshots, spy backstabs...
}

pub trait WeaponDriver {
	pub fn get_entity_classname() -> &'static [&'static str];
}  