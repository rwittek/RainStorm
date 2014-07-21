use sdk;

pub trait BaseWeaponDriver {
	/// For which classnames is this driver applicable?
	/// TODO: this is wrong for things like Amby, which is a "revolver with attributes"
	fn applies_to_entity(ent: &sdk::C_BaseEntity) -> bool;
}

/// Will change soon.
pub trait AimbotWeaponDriver: BaseWeaponDriver {
	fn get_best_hitbox(&self) -> int
}

pub struct SniperRifleDriver;

impl BaseWeaponDriver for SniperRifleDriver {
	fn applies_to_entity(ent: &sdk::C_BaseEntity) -> bool {
		return true; // FIXME
	}
}

impl AimbotWeaponDriver for SniperRifleDriver {
	fn get_best_hitbox(&self) -> int {
		1
	}
}