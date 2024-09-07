use crate::imports::BuildImports::*;

unsafe extern "C" fn status_nana_SpecialS_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_s2") as i64, *FIGHTER_POPO_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s2") as i64, *FIGHTER_POPO_STATUS_WORK_INT_MOT_AIR_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_s1") as i64, *FIGHTER_POPO_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s1") as i64, *FIGHTER_POPO_STATUS_WORK_INT_MOT_AIR_KIND);
    }
    nana_SpecialS_motion_function(fighter, true.into());
    if !StopModule::is_stop(fighter.module_accessor) {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_SPEED_UP_Y_COUNT);
    }
    fighter.global_table[PREV_SUB_STATUS].assign(&L2CValue::Ptr(nana_SpecialS_Prev_Sub_status as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(nana_SpecialS_Main_loop as *const () as _))
}

unsafe extern "C" fn nana_SpecialS_Prev_Sub_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::inc_int(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_SPEED_UP_Y_COUNT);
    0.into()
}

unsafe extern "C" fn nana_SpecialS_motion_function(fighter: &mut L2CFighterCommon, param_1: bool) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_POPO_STATUS_WORK_INT_MOT_KIND);
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_POPO_STATUS_WORK_INT_MOT_AIR_KIND); 
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_END);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_COUPLE);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_SINGLE);
            }
        }
        if motion_kind == mot_kind {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
        }    
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);   
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            fighter.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_END);
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_COUPLE);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_AIR_S_SINGLE);
                }
            }
            if motion_kind == mot_kind {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            }
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);   
        }
    }
    else {  
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END) {
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_END);
        }
        else {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_COUPLE);
            }
            else {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_SINGLE);
            }
        }
        if motion_kind == mot_air_kind {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_AIR_HOP_BUTTON_TRIGGER);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_AIR_HOP_BUTTON_TRIGGER_COUNTER);
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_PHASE_END) {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_END);
            }
            else {
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_COUPLE) {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_COUPLE);
                }
                else {
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_POPO_SPECIAL_S_SINGLE);
                }
            }
            if motion_kind == mot_air_kind {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
            }
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_AIR_HOP_BUTTON_TRIGGER);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_AIR_HOP_BUTTON_TRIGGER_COUNTER);
        }
    }
    0.into()
}

unsafe extern "C" fn nana_SpecialS_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if !fighter.sub_transition_group_check_air_cliff().get_bool() {
            if MotionModule::is_end(fighter.module_accessor) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
            }
            if !StatusModule::is_changing(fighter.module_accessor) {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_SUB_FIGHTER) {
                    if LinkModule::is_link(fighter.module_accessor, *FIGHTER_POPO_LINK_NO_PARTNER) {
                        if nana_SpecialS_motion_function(fighter, false.into()).get_bool() {
                            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2a6fca0d7f));
                        }
                    }
                }
            }
            let trigger_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_AIR_HOP_BUTTON_TRIGGER_COUNTER);
            if 0 < trigger_count {
                WorkModule::dec_int(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_WORK_INT_AIR_HOP_BUTTON_TRIGGER_COUNTER);
                if trigger_count == 0 {
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_POPO_STATUS_SPECIAL_S_FLAG_AIR_HOP_BUTTON_TRIGGER);
                }
            }
            return 0.into();
        }
    }
    else {
        if !fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 0.into();
            }
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
    return 1.into();
}

pub fn install() {
    Agent::new("nana")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_S, status_nana_SpecialS_Main)
    .install();
}