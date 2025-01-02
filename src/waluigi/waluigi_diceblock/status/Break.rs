use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_diceblock_Break_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

unsafe extern "C" fn status_waluigi_diceblock_Break_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
	VisibilityModule::set_whole(weapon.module_accessor, false);
	sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 1.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(waluigi_diceblock_Break_Main_loop as *const () as _))
}

pub unsafe extern "C" fn waluigi_diceblock_Break_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::change_status_force(weapon.module_accessor, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_DIE, false);
	0.into()
}

pub unsafe extern "C" fn status_waluigi_diceblock_Break_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .status(Pre, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, status_waluigi_diceblock_Break_Pre)
	.status(Main, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, status_waluigi_diceblock_Break_Main)
    .status(End, *WEAPON_WALUIGI_DICEBLOCK_STATUS_KIND_BREAK, status_waluigi_diceblock_Break_End)
    .install();
}