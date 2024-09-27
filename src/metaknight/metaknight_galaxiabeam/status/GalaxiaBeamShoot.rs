use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_GalaxiaBeamShoot_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_metaknight_GalaxiaBeamShoot_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("life"));
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("scale"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    ModelModule::set_scale(weapon.module_accessor, scale * 0.001);
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("speed_max"));
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("angle"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().sin() * speed_max * lr;
    let speed_y = angle.to_radians().cos() * speed_max;
    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn status_metaknight_GalaxiaBeamShoot_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let speed_max = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("speed_max"));
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("angle"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let speed_x = angle.to_radians().sin() * speed_max * lr;
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    if GroundModule::is_floor_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        weapon.set_situation(SITUATION_KIND_GROUND.into());
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, 0.0);
        sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, 0.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    }
    weapon.fastshift(L2CValue::Ptr(metaknight_GalaxiaBeamShoot_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_GalaxiaBeamShoot_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    if should_remove_galaxia(weapon)
    || situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind == *SITUATION_KIND_AIR && WorkModule::is_flag(owner_module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_AIR_SPECIAL_GUARD) {
        galaxia_beam_removal(weapon);
    }
    let is_penetration = WorkModule::get_param_int(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("is_penetration"));
    if is_penetration == 0 {
        if should_remove_galaxia_on_hit(weapon) {
            galaxia_beam_hit_removal(weapon);
        }
    }
    else if is_penetration == 1 {
        return 0.into();
    }
    0.into()
}

pub unsafe extern "C" fn should_remove_galaxia(weapon: &mut L2CWeaponCommon) -> bool {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_HIT_WALL)
    || life <= 0 {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn should_remove_galaxia_on_hit(weapon: &mut L2CWeaponCommon) -> bool {
    if AttackModule::is_infliction_status(weapon.module_accessor, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT)
    || StopModule::is_stop(weapon.module_accessor)
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_KOOPAJR_CANNONBALL_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return true;
    }
    return false;
}

pub unsafe extern "C" fn galaxia_beam_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("sys_erace_smoke"), &Vector3f{x: pos.x, y: pos.y, z: pos.z + 5.0}, &Vector3f {x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    EffectModule::req(weapon.module_accessor, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z + 5.0}, &Vector3f {x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

pub unsafe extern "C" fn galaxia_beam_hit_removal(weapon: &mut L2CWeaponCommon) {
    let pos = *PostureModule::pos(weapon.module_accessor);
    EffectModule::req(weapon.module_accessor, Hash40::new("miiswordsman_hensoku_hit"), &Vector3f{x: pos.x, y: pos.y, z: pos.z + 5.0}, &Vector3f {x: 0.0, y: 0.0, z: 0.0}, 1.0, 0, -1, false, 0);
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x18b78d41a0));
    notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
    weapon.pop_lua_stack(1);
}

unsafe extern "C" fn status_metaknight_GalaxiaBeamShoot_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    0.into()
}

unsafe extern "C" fn status_metaknight_GalaxiaBeamShoot_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .status(Pre, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_GalaxiaBeamShoot_Pre)
    .status(Init, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_GalaxiaBeamShoot_Init)
    .status(Main, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_GalaxiaBeamShoot_Main)
    .status(Exec, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_GalaxiaBeamShoot_Exec)
    .status(End, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_GalaxiaBeamShoot_End)
    .install();
}