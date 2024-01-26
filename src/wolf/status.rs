use crate::imports::BuildImports::*; 

#[status_script(agent = "wolf", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_wolf_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_START, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    WorkModule::set_int(fighter.module_accessor, -1, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP_PREV);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    if fighter.global_table[FIGHTER_KIND].get_i32() == *FIGHTER_KIND_WOLF {
        let revert_angle_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("revert_angle_frame"));
        let max_rush_degree = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("max_rush_degree"));
        let min_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("min_stick_y"));
        WorkModule::set_int(fighter.module_accessor, revert_angle_frame, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_FRAME);
        WorkModule::set_float(fighter.module_accessor, max_rush_degree, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
        WorkModule::set_float(fighter.module_accessor, min_stick_y, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    }
    PostureModule::set_stick_lr(fighter.module_accessor, 0.0);
    PostureModule::update_rot_y_lr(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        wolf_SpecialS_motion_air_handler(fighter);
    }
    else {
        wolf_SpecialS_motion_handler(fighter);
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

unsafe fn wolf_SpecialS_Sub_status(fighter: &mut L2CFighterCommon, param_3: L2CValue) -> L2CValue {
    if param_3.get_bool() {
        WorkModule::dec_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STOP_Y_FRAME);
        let int_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if int_step == *FIGHTER_FOX_ILLUSION_STEP_END {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_REVERT_ANGLE_COUNT);
        }
    }
    0.into()
}

unsafe fn wolf_SpecialS_motion_air_handler(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_start"), -1.0, 1.0, 0.0, false, false);
    }
}

unsafe fn wolf_SpecialS_motion_handler(fighter: &mut L2CFighterCommon) {
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_start"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_start"), -1.0, 1.0, 0.0, false, false);
    }
}

pub unsafe extern "C" fn SpecialS_change_motion(fighter: &mut L2CFighterCommon, mot: Hash40) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE) {
        MotionModule::change_motion(fighter.module_accessor, mot, 0.0, 1.0, false, 0.0, false, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
    }
    else {
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, mot, -1.0, 1.0, 0.0, false, false);
    }
}

pub unsafe extern "C" fn SpecialS_air_control(fighter: &mut L2CFighterCommon) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        let air_accel_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_mul"), 0);
        let air_accel_x_add = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_x_add"), 0);
        let illusion_end_control_air_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_control_air_speed_x_mul"));
        sv_kinetic_energy!(controller_set_accel_x_mul, fighter, air_accel_x_mul * illusion_end_control_air_speed_x_mul);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, air_accel_x_add * illusion_end_control_air_speed_x_mul);
        let air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_x_stable"), 0);
        let illusion_end_control_air_speed_x_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("illusion_end_control_air_speed_x_stable"));
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_speed_x_stable * illusion_end_control_air_speed_x_stable, 0.0);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn wolf_SpecialS_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut cont = true;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) && 
    fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
        return 0.into();
    }
    let int_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
    if !StatusModule::is_changing(fighter.module_accessor) { 
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if int_step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
                WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                cont = true;
            }
            else if int_step == *FIGHTER_FOX_ILLUSION_STEP_END {
                if MotionModule::is_end(fighter.module_accessor) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                    return 0.into();
                }
            }
            if fighter.global_table[SITUATION_KIND] == *SITUATION_KIND_GROUND {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_landing"), 0.0, 1.0, false, 0.0, false, false);
                return 0.into();
            }
        }
        else if int_step == *FIGHTER_FOX_ILLUSION_STEP_RUSH || int_step == *FIGHTER_FOX_ILLUSION_STEP_START {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND || MotionModule::is_end(fighter.module_accessor) {
                cont = true;
            }
        }
    }
    else {
        if int_step == *FIGHTER_FOX_ILLUSION_STEP_FORCE_END {
            WorkModule::set_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STEP_END, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
            cont = true;
        }
        else if int_step == *FIGHTER_FOX_ILLUSION_STEP_END {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
                return 0.into();
            }
            if MotionModule::is_end(fighter.module_accessor) && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
                return 0.into();
            }
        }
        else if int_step == *FIGHTER_FOX_ILLUSION_STEP_RUSH || int_step == *FIGHTER_FOX_ILLUSION_STEP_START {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR || MotionModule::is_end(fighter.module_accessor) {
                cont = true;
            }
        }
    }
    if cont {
        if !StatusModule::is_changing(fighter.module_accessor) {
            if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_RUSH_FORCE_END)
            && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_HIT_SHIELD_TO_END) {
                if MotionModule::is_end(fighter.module_accessor) {
                    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLAG_CONTINUE);
                }
            }
        }
        let int_step = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_INT_STEP);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if int_step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                SpecialS_change_motion(fighter, Hash40::new("special_air_s_end"));
                SpecialS_air_control(fighter);
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
            else if int_step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
                GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_FOX_CLIFF_HANG_DATA_SPECIAL_S as u32);
                SpecialS_change_motion(fighter, Hash40::new("special_air_s"));
                fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
            }
            else if int_step == *FIGHTER_FOX_ILLUSION_STEP_START {
                wolf_SpecialS_motion_air_handler(fighter);
            }
        }
        else {
            fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
            if int_step == *FIGHTER_FOX_ILLUSION_STEP_END {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
                SpecialS_change_motion(fighter, Hash40::new("special_s_end"));
                SpecialS_air_control(fighter);
            }
            else if int_step == *FIGHTER_FOX_ILLUSION_STEP_RUSH {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
                sv_kinetic_energy!(friction_off, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                let motion_mul = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
                sv_kinetic_energy!(set_speed_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION, motion_mul);
                SpecialS_change_motion(fighter, Hash40::new("special_s"));
            }
            else if int_step == *FIGHTER_FOX_ILLUSION_STEP_START {
                wolf_SpecialS_motion_handler(fighter);
            }
        }
    }
    let max_deg = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MAX_RUSH_DEGREE);
    let min_stick = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_MIN_STICK);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let angle = stick_y.signum() * max_deg * f32::max(stick_y.abs() - min_stick, 0.0) / (1.0 - min_stick);
    PostureModule::set_rot(fighter.module_accessor, &Vector3f{x: max_deg, y: 0.0, z: 0.0}, 0);
    WorkModule::set_float(fighter.module_accessor, angle, *FIGHTER_FOX_ILLUSION_STATUS_WORK_ID_FLOAT_RUSH_DEGREE);
    0.into()
}

#[status_script(agent = "wolf", status = FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_wolf_SpecialHiRushEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLOAT_REVERT_DEGREE);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_KICK);
    if fighter.global_table[SITUATION_KIND].get_i32() != SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fall"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_landing"), 0.0, 1.0, false, 0.0, false, false);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_FIRE_TRANSITION_TERM_ID_WAIT);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(wolf_SpecialHiRushEnd_Main_Loop as *const () as _))
}

unsafe fn wolf_SpecialHiRushEnd_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[STATUS_KIND_INTERRUPT].get_i32() == *FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_AIR)
    && fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        let normal_x = GroundModule::get_touch_normal_x(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let normal_y = GroundModule::get_touch_normal_y(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let angle = sv_math::vec2_angle(normal_x, normal_y, speed_x, speed_y);
        let fire_crush_angle = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fire_crush_angle"));
        let rad = (fire_crush_angle + 90.0).to_radians();
        if rad < angle {
            fighter.change_status(FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND.into(), false.into());
        }
    }
}

unsafe extern "C" fn wolf_SpecialHiRushEnd_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let revert_dir_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("revert_dir_frame"));
    let dir = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLOAT_DIR);
    let revert_degree = WorkModule::get_float(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLOAT_REVERT_DEGREE);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() { 
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    let revert_degree_set = revert_dir_frame as f32 / dir;
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_KICK) {
        if revert_degree == 0.0 {
            WorkModule::set_float(fighter.module_accessor, revert_degree_set, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLOAT_REVERT_DEGREE);
        }
        if prev_situation_kind == *SITUATION_KIND_GROUND {
            if situation_kind == *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
                return 0.into();
            }
        }
        if situation_kind == *SITUATION_KIND_GROUND && MotionModule::is_end(fighter.module_accessor) {
            if prev_situation_kind != *SITUATION_KIND_AIR {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
            }
            return 0.into();
        }
        if prev_situation_kind == *SITUATION_KIND_AIR {
            if situation_kind == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                return 0.into();
            }
        }
        if situation_kind == *SITUATION_KIND_AIR && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
            return 0.into();
        }
    }
    else {
        if prev_situation_kind != *SITUATION_KIND_GROUND {
            if prev_situation_kind == *SITUATION_KIND_AIR {
                if situation_kind == *SITUATION_KIND_GROUND {
                    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
                    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
                    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_FOX_FIRE_TRANSITION_TERM_ID_WAIT);
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_landing"), -1.0, 1.0, 0.0, false, false);
                    if revert_degree == 0.0 {
                        WorkModule::set_float(fighter.module_accessor, revert_degree_set, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLOAT_REVERT_DEGREE);
                        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_FOX_FIRE_STATUS_WORK_ID_FLAG_WOLF_KICK);
                    }
                }
            }
        }
        else {
            if situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_FOX_FIRE_TRANSITION_TERM_ID_WAIT);
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi_fall"), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    if !fighter.global_table[IS_STOP].get_bool() {
        wolf_SpecialHiRushEnd_handler(fighter);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_wolf_SpecialHiRushEnd_Main,
        //status_wolf_SpecialS_Main
    );
}
