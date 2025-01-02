use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_diceblock_Die_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

unsafe extern "C" fn status_waluigi_diceblock_Die_Main(weapon: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 10, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	VisibilityModule::set_whole(weapon.module_accessor, false);
	sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
	weapon.fastshift(L2CValue::Ptr(waluigi_diceblock_Die_Main_loop as *const () as _))
}

pub unsafe extern "C" fn waluigi_diceblock_Die_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	if life < 0 {
		notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
	}
	0.into()
}

pub unsafe extern "C" fn status_waluigi_diceblock_Die_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::off_flag(weapon.module_accessor, *FIGHTER_WALUIGI_INSTANCE_WORK_ID_FLAG_DICEBLOCK_SELECT_NUM);
    0.into()
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .status(Pre, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_DIE, status_waluigi_diceblock_Die_Pre)
	.status(Main, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_DIE, status_waluigi_diceblock_Die_Main)
	.status(End, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_DIE, status_waluigi_diceblock_Die_End)
    .install();
}