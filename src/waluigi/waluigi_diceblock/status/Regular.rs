use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_diceblock_Regular_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	if life < 20 {
		StatusModule::change_status_force(weapon.module_accessor, WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, false);
	}
	0.into()
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .status(Exec, *WEAPON_MARIO_FIREBALL_STATUS_KIND_REGULAR, status_waluigi_diceblock_Regular_Exec)
    .install();
}