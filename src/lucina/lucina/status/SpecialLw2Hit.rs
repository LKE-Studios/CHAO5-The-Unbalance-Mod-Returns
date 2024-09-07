use crate::imports::BuildImports::*;

unsafe extern "C" fn status_lucina_SpecialLw2Hit_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_LW as u64 | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK as u64), 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_LW as u32, 0);
    0.into()
}

unsafe extern "C" fn status_lucina_SpecialLw2Hit_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let x = 0.0;
    let y = 0.0;
    let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let mut power;
    let mut vector = fighter.Vector2__create(x.into(), y.into());
    let get_stop_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    let get_gravity_energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) as *mut smash::app::FighterKineticEnergyGravity;
    let attack_power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    let attack_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_mul"));
    let attack_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_max"));
    let attack_max_for_enemy = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_max_for_enemy"));
    let attack_power_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_power_limit"));
    let start_mul_spd_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_mul_spd_x"));
    let start_air_acl_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("start_air_acl_x"));
    let attack_max_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_max_y"));
    let attack_acl_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_lw"), hash40("attack_acl_y"));
    let vec_x = vector["x"].get_f32();
    let vec_y = vector["y"].get_f32();
    vector["x"].assign(&L2CValue::F32(sum_speed));
    vector["y"].assign(&L2CValue::F32(sum_speed));
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind != *FIGHTER_STATUS_KIND_SPECIAL_LW {
        power = attack_power * attack_mul;
        if status_kind != *FIGHTER_MARTH_STATUS_KIND_SPECIAL_LW_HIT {
            return 0.into();
        }
        if power < attack_power_limit {
            power = attack_power_limit;
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_IS_ATTACK_ENEMY) {
            if attack_max_for_enemy < power {
                power = attack_max_for_enemy;
            }
        }
        else {
            if attack_max < power {
                power = attack_max;
            }
        }
        WorkModule::set_float(fighter.module_accessor, power, *FIGHTER_MARTH_STATUS_SPECIAL_LW_WORK_FLOAT_ATTACK_POWER);
    }
    else {
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MARTH_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        }
        else {
            vector["x"].assign(&L2CValue::F32(vec_x * start_mul_spd_x));
            vector["y"].assign(&L2CValue::F32(0.0));
            KineticEnergy::reset_energy(get_stop_energy as *mut smash::app::KineticEnergy, *ENERGY_STOP_RESET_TYPE_AIR, &Vector2f{x: vec_x, y: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
            KineticEnergy::reset_energy(get_stop_energy as *mut smash::app::KineticEnergy, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, &Vector2f{x: 0.0, y: vec_y}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, fighter.module_accessor);
            KineticEnergyNormal::set_brake(get_stop_energy as *mut smash::app::KineticEnergyNormal, &Vector2f{x: start_air_acl_x, y: 0.0});
            FighterKineticEnergyGravity::set_accel(get_gravity_energy, -attack_acl_y);
            FighterKineticEnergyGravity::set_stable_speed(get_gravity_energy, attack_max_y);
            KineticEnergy::enable(get_stop_energy as *mut smash::app::KineticEnergy);
            KineticEnergy::enable(get_gravity_energy as *mut smash::app::KineticEnergy);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
            KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MARTH_STATUS_SPECIAL_LW_WORK_INT_SITUATION_PREV);
        }
    }
    0.into()
}

unsafe extern "C" fn status_lucina_SpecialLw2Hit_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    lucina_SpecialLw2Hit_motion_helper(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(lucina_SpecialLw2Hit_Main_loop as *const () as _))
}

unsafe extern "C" fn lucina_SpecialLw2Hit_motion_helper(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_air_lw_hit_2"), -1.0, 1.0, 0.0);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_IGNORE_NORMAL);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("special_lw_hit_2"), -1.0, 1.0, 0.0);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_hit_2"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
    }
}

unsafe extern "C" fn lucina_SpecialLw2Hit_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND && 
        fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            lucina_SpecialLw2Hit_motion_helper(fighter);
        }
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR && 
        fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            lucina_SpecialLw2Hit_motion_helper(fighter);
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

unsafe extern "C" fn status_lucina_SpecialLw2Hit_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("lucina")
    .status(Pre, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2, status_lucina_SpecialLw2Hit_Pre)
    .status(Init, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2, status_lucina_SpecialLw2Hit_Init)
    .status(Main, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2, status_lucina_SpecialLw2Hit_Main)
    .status(End, FIGHTER_LUCINA_STATUS_KIND_SPECIAL_LW_HIT2, status_lucina_SpecialLw2Hit_End)
    .install();
}