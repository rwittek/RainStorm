pub trait WeaponDriver {
	/// For which classnames is this driver applicable?
	fn get_classnames() -> &'static [&'static str];
}

/// Will change soon.
pub trait AimbotWeaponDriver {
	fn get_best_hitb(&self) -> int
}