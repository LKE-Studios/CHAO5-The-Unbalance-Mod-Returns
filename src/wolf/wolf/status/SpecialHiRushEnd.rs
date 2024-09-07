use crate::imports::BuildImports::*; 

pub unsafe extern "C" fn status_wolf_SpecialHiRushEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
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

unsafe extern "C" fn wolf_SpecialHiRushEnd_handler(fighter: &mut L2CFighterCommon) {
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
    Agent::new("wolf")
    .status(Main, *FIGHTER_WOLF_STATUS_KIND_SPECIAL_HI_RUSH_END, status_wolf_SpecialHiRushEnd_Main)
    .install();
}
