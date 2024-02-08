use crate::imports::BuildImports::*;

unsafe extern "C" fn status_captain_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(captain_SpecialSEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn captain_SpecialSEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            WorkModule::get_int(fighter.module_accessor, *FIGHTER_CAPTAIN_STATUS_WORK_ID_INT_FALCON_KNUCKLE_START_SITUATION);
            if MotionModule::is_end(fighter.module_accessor) {
                if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING) {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                            fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                        }
                    }
                }
                if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
        }
    }
    return 1.into();
}

pub fn install() {
    Agent::new("captain")
    .status(Main, *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_S_END, status_captain_SpecialSEnd_Main)
    .install();
}