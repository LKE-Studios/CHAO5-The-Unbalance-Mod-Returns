use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_fire_FlyS_Pre(weapon: &mut L2CFighterCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        let lr = PostureModule::lr(owner_module_accessor);
        let owner_pos_x = PostureModule::pos_x(owner_module_accessor);
        let owner_pos_y = PostureModule::pos_y(owner_module_accessor);
        let owner_pos_z = PostureModule::pos_z(owner_module_accessor);
        StatusModule::init_settings(weapon.module_accessor, SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(0), false, 0, 0, 0, 0);
        PostureModule::set_pos(weapon.module_accessor, &Vector3f{x: owner_pos_x + (8.0 * lr), y: owner_pos_y - 6.0, z: owner_pos_z});
        0.into()
    }
    else {
        original_status(Pre, weapon, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S)(weapon)
    }
}

unsafe extern "C" fn status_bandana_fire_FlyS_Main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        let life_s = WorkModule::get_param_int(weapon.module_accessor, hash40("param_fire"), hash40("life_s"));
        WorkModule::set_int(weapon.module_accessor, life_s, *WEAPON_INSTANCE_WORK_ID_INT_INIT_LIFE);
        WorkModule::set_int(weapon.module_accessor, life_s, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
        MotionModule::change_motion(weapon.module_accessor, Hash40::new("special_n1"), 0.0, 1.0, false, 0.0, false, false);
        let lr = PostureModule::lr(weapon.module_accessor);
        let accel_x_s = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("accel_x_s"));
        let max_speed_x_s = WorkModule::get_param_float(weapon.module_accessor, hash40("param_fire"), hash40("max_speed_x_s"));
        sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s * lr, 0.0);
        sv_kinetic_energy!(set_accel, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, accel_x_s * lr, 0.0);
        sv_kinetic_energy!(set_limit_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, max_speed_x_s, -1.0);
        let charge = WorkModule::get_int(owner_module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_CHARGE);
        let mut store_frame = WorkModule::get_int(owner_module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_SPECIAL_S_STORE_FRAME);
        store_frame = charge;
        let pearl_id = (*(weapon.module_accessor)).battle_object_id as i64;
        WorkModule::set_int64(owner_module_accessor, pearl_id, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_FIRE_ID);
        weapon.fastshift(L2CValue::Ptr(bandana_fire_FlyS_Main_loop as *const () as _))
    }
    else {
        original_status(Main, weapon, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S)(weapon)
    }
}

pub unsafe extern "C" fn bandana_fire_status_helper(weapon: &mut L2CWeaponCommon, status: i32) -> L2CValue {
    let life = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    if life <= 0 || WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL) 
    && weapon.global_table[CURRENT_FRAME].get_f32() <= 2.0 {
        weapon.change_status(status.into(), false.into());
        return 1.into()
    }
    else {
        if weapon.sub_ground_module_is_touch_all_consider_speed().get_bool() {
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_HIT_WALL);
            if current_frame > 1.0 {
                weapon.change_status(status, false.into());
                return 1.into()
            }
            StopModule::set_other_stop(weapon.module_accessor, 2, StopOtherKind(0));
        }
    }
    if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_EDGE_FIRE_INSTANCE_WORK_ID_FLAG_ATTACK) {
        return 0.into()
    }
    weapon.change_status(status, false.into());
    1.into()
}

unsafe extern "C" fn bandana_fire_FlyS_Main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    bandana_fire_status_helper(weapon, WEAPON_EDGE_FIRE_STATUS_KIND_BURST_S.into());
    0.into()
}

unsafe extern "C" fn status_bandana_fire_FlyS_End(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(weapon.module_accessor, Hash40::new("se_edge_special_n04_03"), 0);
        0.into()
    }
    else {
        original_status(End, weapon, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S)(weapon)
    }
}

pub fn install() {
    Agent::new("edge_fire")
    .status(Pre, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, status_bandana_fire_FlyS_Pre)
    .status(Main, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, status_bandana_fire_FlyS_Main)
    .status(End, *WEAPON_EDGE_FIRE_STATUS_KIND_FLY_S, status_bandana_fire_FlyS_End)
    .install();
}