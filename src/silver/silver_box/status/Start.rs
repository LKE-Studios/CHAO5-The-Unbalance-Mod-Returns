use crate::imports::BuildImports::*;

unsafe extern "C" fn status_silver_box_Start_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_silver_box_Start_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_silver_box"), hash40("life"));
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_silver_box"), hash40("speed"));
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_silver_box"), hash40("angle"));
    let lr = PostureModule::lr(weapon.module_accessor);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed_x = angle.to_radians().sin() * speed * lr;
    let speed_y = angle.to_radians().cos() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn status_silver_box_Start_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_silver_box"), hash40("scale"));
    ModelModule::set_scale(weapon.module_accessor, scale);
    AttackModule::set_attack_scale(weapon.module_accessor, scale, true);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(silver_box_Start_Main_loop as *const () as _))
}

unsafe extern "C" fn silver_box_Start_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
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

unsafe extern "C" fn status_silver_box_Start_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
	WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
	0.into()
}

unsafe extern "C" fn status_silver_box_Start_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("mewtwo_box")
	.status(Pre, WEAPON_SILVER_BOX_STATUS_KIND_START, status_silver_box_Start_Pre)
    .status(Init, WEAPON_SILVER_BOX_STATUS_KIND_START, status_silver_box_Start_Init)
	.status(Main, WEAPON_SILVER_BOX_STATUS_KIND_START, status_silver_box_Start_Main)
    .status(Exec, WEAPON_SILVER_BOX_STATUS_KIND_START, status_silver_box_Start_Exec)
    .status(End, WEAPON_SILVER_BOX_STATUS_KIND_START, status_silver_box_Start_End)
    .install();
}