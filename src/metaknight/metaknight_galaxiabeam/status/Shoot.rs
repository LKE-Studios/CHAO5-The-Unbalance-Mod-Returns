use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_galaxiabeam_Shoot_Pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_Shoot_Init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let life = WorkModule::get_param_int(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("life"));
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
    WorkModule::set_int(weapon.module_accessor, life, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    let speed = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("speed"));
    let lr = PostureModule::lr(weapon.module_accessor);
    let angle = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("angle"));
    weapon.clear_lua_stack();
    let speed_x = angle.to_radians().sin() * speed * lr;
    let speed_y = angle.to_radians().cos() * speed;
    sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, -speed_y);
    sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, 0.0);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_Shoot_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let scale = WorkModule::get_param_float(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("scale"));
    ModelModule::set_scale(weapon.module_accessor, scale * 0.5);
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("shoot"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(metaknight_galaxiabeam_Shoot_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_galaxiabeam_Shoot_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let situation_kind = weapon.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = weapon.global_table[PREV_SITUATION_KIND].get_i32();
    let is_penetration = WorkModule::get_param_int(weapon.module_accessor, hash40("param_galaxiabeam"), hash40("is_penetration"));
    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) {
        weapon.change_status(WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_END.into(), false.into())
    }
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_Shoot_Exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    0.into()
}

unsafe extern "C" fn status_metaknight_galaxiabeam_Shoot_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    EffectModule::kill_kind(weapon.module_accessor, Hash40::new("miiswordsman_final_edge_yellow"), false, false);
    0.into()
}

pub fn install() {
    Agent::new("metaknight_galaxiabeam")
    .status(Pre, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_galaxiabeam_Shoot_Pre)
    .status(Init, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_galaxiabeam_Shoot_Init)
    .status(Main, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_galaxiabeam_Shoot_Main)
    .status(Exec, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_galaxiabeam_Shoot_Exec)
    .status(End, WEAPON_METAKNIGHT_GALAXIA_BEAM_STATUS_KIND_SHOOT, status_metaknight_galaxiabeam_Shoot_End)
    .install();
}