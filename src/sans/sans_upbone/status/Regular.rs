use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_upbone_Regular_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_sans_upbone_Regular_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_module_accessor = sv_battle_object::module_accessor(owner);
	let lr = PostureModule::lr(owner_module_accessor);
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_upbone"), hash40("life"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_upbone"), hash40("speed"));  
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed * lr, 0.0);
    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 2.0, 2.0);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    0.into()
}

unsafe extern "C" fn status_sans_upbone_Regular_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_upbone"), hash40("scale"));  
    ModelModule::set_scale(weapon.module_accessor, scale);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(sans_upbone_Regular_Main_loop as *const () as _))
}

unsafe extern "C" fn sans_upbone_Regular_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

unsafe extern "C" fn status_sans_upbone_Regular_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn status_sans_upbone_Regular_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("palutena_upbone")
    .status(Pre, *WEAPON_ROCKMAN_ROCKBUSTER_STATUS_KIND_REGULAR, status_sans_upbone_Regular_Pre)
    .status(Init, *WEAPON_ROCKMAN_ROCKBUSTER_STATUS_KIND_REGULAR, status_sans_upbone_Regular_Init)
    .status(Main, *WEAPON_ROCKMAN_ROCKBUSTER_STATUS_KIND_REGULAR, status_sans_upbone_Regular_Main)
    .status(Exec, *WEAPON_ROCKMAN_ROCKBUSTER_STATUS_KIND_REGULAR, status_sans_upbone_Regular_Exec)
    .status(End, *WEAPON_ROCKMAN_ROCKBUSTER_STATUS_KIND_REGULAR, status_sans_upbone_Regular_End)
    .install();
}