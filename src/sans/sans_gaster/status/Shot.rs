use crate::imports::BuildImports::*;

pub static gaster_end_frame : f32 = 79.0;

unsafe extern "C" fn status_sans_gaster_Shot_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLAG, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_INT, *WEAPON_STATUS_WORK_KEEP_FLAG_ALL_FLOAT, *FS_SUCCEEDS_KEEP_ATTACK as i32);
    0.into()
}

unsafe extern "C" fn status_sans_gaster_Shot_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    if !WorkModule::is_flag(owner_module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_SIZE) {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("fire"), 1.0, 1.0, false, 0.0, false, false);
        ModelModule::set_scale(weapon.module_accessor, 1.15);
        AttackModule::set_attack_scale(weapon.module_accessor, 1.15, true);
    }
    else {
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("final_fire"), 0.0, 1.0, false, 0.0, false, false);
        ModelModule::set_scale(weapon.module_accessor, 6.0);
        AttackModule::set_attack_scale(weapon.module_accessor, 6.0, true);
    }
    weapon.fastshift(L2CValue::Ptr(sans_gaster_Shot_Main_loop as *const () as _))
}

unsafe extern "C" fn sans_gaster_Shot_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if MotionModule::frame(weapon.module_accessor) >= gaster_end_frame {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    0.into()
}

unsafe extern "C" fn status_sans_gaster_Shot_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	0.into()
}

unsafe extern "C" fn status_sans_gaster_Shot_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
	0.into()
}

pub fn install() {
    Agent::new("palutena_gaster")
	.status(Pre, *WEAPON_SANS_GASTER_STATUS_KIND_SHOT, status_sans_gaster_Shot_Pre)
	.status(Main, *WEAPON_SANS_GASTER_STATUS_KIND_SHOT, status_sans_gaster_Shot_Main)
    .status(Exec, *WEAPON_SANS_GASTER_STATUS_KIND_SHOT, status_sans_gaster_Shot_Exec)
    .status(End, *WEAPON_SANS_GASTER_STATUS_KIND_SHOT, status_sans_gaster_Shot_End)
    .install();
}