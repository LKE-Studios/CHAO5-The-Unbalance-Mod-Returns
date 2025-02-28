use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_downbone_Stick_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_sans_downbone_Stick_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let stick_frame = WorkModule::get_param_int(weapon.module_accessor, hash40("param_downbone"), hash40("stick_frame"));
    WorkModule::set_int(weapon.module_accessor, stick_frame, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, stick_frame, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn status_sans_downbone_Stick_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x220cea5125));
    weapon.pop_lua_stack(1);
    GroundModule::attach(weapon.module_accessor, GroundTouchFlag(*GROUND_TOUCH_FLAG_ALL));
    GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("stick"), 0.0, 1.0, false, 0.0, false, false);
    if !StopModule::is_stop(weapon.module_accessor) {
        sans_downbone_Stick_Sub_status(weapon, false.into());
    }
    weapon.global_table[SUB_STATUS].assign(&L2CValue::Ptr(sans_downbone_Stick_Sub_status as *const () as _));
    weapon.fastshift(L2CValue::Ptr(sans_downbone_Stick_Main_loop as *const () as _))
}

unsafe extern "C" fn sans_downbone_Stick_Sub_status(weapon: &mut L2CWeaponCommon, life_dec: L2CValue) -> L2CValue {
    if life_dec.get_bool() {
        WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    }
    0.into()
}

unsafe extern "C" fn sans_downbone_Stick_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_sans_downbone_Stick_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("palutena_downbone")
    .status(Pre, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_STICK, status_sans_downbone_Stick_Pre)
    .status(Init, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_STICK, status_sans_downbone_Stick_Init)
    .status(Main, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_STICK, status_sans_downbone_Stick_Main)
    .status(End, *WEAPON_ROCKMAN_HARDKNUCKLE_STATUS_KIND_STICK, status_sans_downbone_Stick_End)
    .install();
}