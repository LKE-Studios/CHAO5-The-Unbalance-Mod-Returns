use crate::imports::BuildImports::*; 

unsafe extern "C" fn status_wolf_SpecialS_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_S | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_S as u32, 0);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialS_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let illusion_stop_y_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame"));
    WorkModule::set_int(fighter.module_accessor, illusion_stop_y_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let illusion_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_start_x_mul"));
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ENERGY_STOP_RESET_TYPE_AIR, sum_speed_x * illusion_start_x_mul, 0.0, 0.0, 0.0, 0.0);
        let illusion_accel_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_accel_x"));
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, illusion_accel_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) != 0 {
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0.0);
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        }
    }
    else {
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * illusion_start_x_mul, 0.0);
    }
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    let revert_angle_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("revert_angle_frame"));
    let max_rush_degree = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("max_rush_degree"));
    let min_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("min_stick_y"));
    WorkModule::set_int(fighter.module_accessor, revert_angle_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    WorkModule::set_float(fighter.module_accessor, max_rush_degree, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    WorkModule::set_float(fighter.module_accessor, min_stick_y, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
    }
    let illusion_rush_speed_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_rush_speed_mul"));
    let illusion_rush_speed_mul_power_up = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_rush_speed_mul_power_up"));
    WorkModule::set_float(fighter.module_accessor, illusion_rush_speed_mul * illusion_rush_speed_mul_power_up, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
    if !StopModule::is_stop(fighter.module_accessor) {
        wolf_SpecialS_Sub_status(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(wolf_SpecialS_Sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_SpecialS_Main_Loop as *const () as _))
}

unsafe extern "C" fn wolf_SpecialS_Sub_status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    }
    0.into()
}

unsafe extern "C" fn wolf_SpecialS_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    if !StatusModule::is_changing(fighter.module_accessor) && MotionModule::is_end(fighter.module_accessor) || StatusModule::is_situation_changed(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_WOLF_STATUS_KIND_SPECIAL_S_RUSH.into(), false.into());
            return 0.into();
        }
        else if StatusModule::is_situation_changed(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            } 
            else {
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            }
            fighter.sub_change_kinetic_type_by_situation(FIGHTER_KINETIC_TYPE_GROUND_STOP.into(), FIGHTER_KINETIC_TYPE_AIR_STOP.into());
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
            fighter.sub_change_motion_by_situation(L2CValue::Hash40s("special_s_start"), L2CValue::Hash40s("special_air_s_start"), true.into());
        }
    }
    let max_deg = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    let min_stick = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let angle = stick_y.signum() * max_deg * f32::max(stick_y.abs() - min_stick, 0.0) / (1.0 - min_stick);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialS_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
        return 0.into();
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME) == 0 
    && WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_stop_y_frame")) != 0 {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, 0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let illusion_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_accel_y"));
        sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -illusion_accel_y);
    }
    0.into()
}

unsafe extern "C" fn status_wolf_SpecialS_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("wolf")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_S, status_wolf_SpecialS_Pre)
    .status(Init, *FIGHTER_STATUS_KIND_SPECIAL_S, status_wolf_SpecialS_Init)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_wolf_SpecialS_Main)
    .status(Exec, *FIGHTER_STATUS_KIND_SPECIAL_S, status_wolf_SpecialS_Exec)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_S, status_wolf_SpecialS_End)
    .install();
}