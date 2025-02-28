use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_sidebone_Start_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_sans_sidebone_Start_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_sidebone"), hash40("speed_min"));
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -speed);
    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, -1.0, -1.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn status_sans_sidebone_Start_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(sans_sidebone_Start_Main_loop as *const () as _))
}

unsafe extern "C" fn sans_sidebone_Start_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let remaining_life = life <= 0;
    if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    }
    if !remaining_life {
        if !GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
            return 0.into();
        }
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
        weapon.pop_lua_stack(1);
        if remaining_life {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            weapon.pop_lua_stack(1);
            return 0.into();
        }
    }
    else {
        notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
        weapon.pop_lua_stack(1);
    }
    0.into()
}

unsafe extern "C" fn status_sans_sidebone_Start_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

pub fn install() {
    Agent::new("palutena_sidebone")
    .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_sans_sidebone_Start_Pre)
    .status(Init, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_sans_sidebone_Start_Init)
    .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_sans_sidebone_Start_Main)
    .status(Exec, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, status_sans_sidebone_Start_Exec)
    .install();
}