use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_box_Break_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
	StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

unsafe extern "C" fn status_silver_box_Break_Main(weapon: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(weapon.module_accessor, 10, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	VisibilityModule::set_whole(weapon.module_accessor, false);
	sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("break"), 1.0, 1.0, false, 0.0, false, false);
	weapon.fastshift(L2CValue::Ptr(silver_box_Break_Main_loop as *const () as _))
}

pub unsafe extern "C" fn silver_box_Break_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
	if life < 0 {
		notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
	}
	0.into()
}

pub unsafe extern "C" fn status_silver_box_Break_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("dolly_box")
    .status(Pre, WEAPON_SILVER_BOX_STATUS_KIND_BREAK, status_silver_box_Break_Pre)
	.status(Main, WEAPON_SILVER_BOX_STATUS_KIND_BREAK, status_silver_box_Break_Main)
    .status(End, WEAPON_SILVER_BOX_STATUS_KIND_BREAK, status_silver_box_Break_End)
    .install();
}