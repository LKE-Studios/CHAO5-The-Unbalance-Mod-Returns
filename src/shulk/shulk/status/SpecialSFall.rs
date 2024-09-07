use crate::imports::BuildImports::*;

unsafe extern "C" fn status_shulk_SpecialSFall_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), false, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, (*FS_SUCCEEDS_KEEP_EFFECT | *FS_SUCCEEDS_KEEP_ATTACK));
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_shulk_SpecialSFall_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    AttackModule::set_accept_no_lr(fighter.module_accessor, 0, true);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let cut_fall_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cut_fall_accel_y"));
    let cut_fall_speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cut_fall_speed_y_max"));
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y);
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -cut_fall_accel_y);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, cut_fall_speed_y_max);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, cut_fall_speed_y_max);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, ENERGY_MOTION_RESET_TYPE_GROUND_TRANS, 0.0, 0.0, 0.0, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_fall"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(shulk_SpecialSFall_Main_loop as *const () as _))
}

unsafe extern "C" fn shulk_SpecialSFall_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let cut_fall_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cut_fall_accel_y"));
    let cut_fall_speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("cut_fall_speed_y_max"));
    let control_speed_limit = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("control_speed_limit"));
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -cut_fall_accel_y);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, cut_fall_speed_y_max);
    sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_speed_limit, 0.0);
    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, control_speed_limit, 0.0);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        fighter.change_status(FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_LANDING.into(), false.into());
        return 0.into();
    }
    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("shulk")
    .status(Pre, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, status_shulk_SpecialSFall_Pre)
    .status(Main, *FIGHTER_SHULK_STATUS_KIND_SPECIAL_S_FALL, status_shulk_SpecialSFall_Main)
    .install();
}