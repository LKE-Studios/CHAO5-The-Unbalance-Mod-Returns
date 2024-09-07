use crate::imports::BuildImports::*; 

pub unsafe extern "C" fn status_younglink_SpecialHiEnd_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mask_flag = (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64;
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_LINK_SPECIAL_HI_END_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, mask_flag, 0, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_younglink_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_hi"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(younglink_SpecialHiEnd_Main_Loop as *const () as _))
}

unsafe extern "C" fn younglink_situation_helper(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_changing(fighter.module_accessor) {
        return 1.into()
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND] == SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_AIR {
            return 1.into()
        }
        if fighter.global_table[PREV_SITUATION_KIND] != SITUATION_KIND_GROUND && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            return 1.into()
        }
    }
    return 0.into()
}

unsafe extern "C" fn younglink_SpecialHiEnd_handler(fighter: &mut L2CFighterCommon) {
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

unsafe extern "C" fn younglink_SpecialHiEnd_Main_Loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut motion_value = 0.55;
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) || (CancelModule::is_enable_cancel(fighter.module_accessor) && !fighter.sub_wait_ground_check_common(false.into()).get_bool() && !fighter.sub_air_check_fall_common().get_bool()) {
        if younglink_situation_helper(fighter).get_bool() {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if !StatusModule::is_changing(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL.into(), false.into());
                    return 1.into()
                }
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST) {
                    MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_hi"), -1.0, 1.0, 0.0, false, false);
                }
                else {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_END_FLAG_FIRST);
                }
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
                fighter.shift(L2CValue::Ptr(sub_younglink_SpecialHiEnd_Main as *const () as _));
                return 0.into()
            }
        }
        if frame < 46.0 {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f{x: motion_value * stick_x.signum(), y: 0.0, z: 0.0});
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
                return 1.into()
            }
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            if MotionModule::is_end(fighter.module_accessor) {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                return 1.into()
            }
        }
    }
    0.into()
}

unsafe extern "C" fn sub_younglink_SpecialHiEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    GroundModule::set_cliff_check(fighter.module_accessor, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES));
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL_SPECIAL) {
        if !MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
                if younglink_situation_helper(fighter).get_bool() {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
                    return 1.into()
                }
            }
            else {
                if fighter.sub_transition_group_check_air_cliff().get_bool() {
                    return 1.into()
                }
            }
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
            return 1.into()
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) && MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 1.into()
        }
        if younglink_situation_helper(fighter).get_bool() && fighter.global_table[SITUATION_KIND] == SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), true.into());
            return 1.into()
        }
    }
    return 0.into()
}

pub fn install() {
    Agent::new("younglink")
    .status(Pre, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, status_younglink_SpecialHiEnd_Pre)
    .status(Main, *FIGHTER_LINK_STATUS_KIND_SPECIAL_HI_END, status_younglink_SpecialHiEnd_Main)
    .install();
}
