use crate::imports::BuildImports::*;

unsafe extern "C" fn status_ike_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
    ike_special_s_end_motion_handler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(ike_special_s_end_main_loop as *const () as _))
}

unsafe extern "C" fn ike_special_s_end_motion_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_s_end"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_s_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
        }
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_NONE.into());
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_s_end"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_end"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_CONTINUE_MOT);
        }
    }
}

unsafe extern "C" fn ike_special_s_end_status_handler(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_IKE_STATUS_SPECIAL_S_FLAG_END_NO_LANDING) {
        if FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_end"), true) - frame <= 0.0 {
            WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        ike_special_s_end_motion_handler(fighter);
    }
}

unsafe extern "C" fn ike_special_s_end_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            if StatusModule::is_changing(fighter.module_accessor) {
                if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                        ike_special_s_end_status_handler(fighter);
                    }
                }
                else {
                    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR {
                        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                            ike_special_s_end_status_handler(fighter);
                        }
                    }
                }
            }
            if GroundModule::is_status_cliff(fighter.module_accessor) {
                fighter.change_status(FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END.into(), true.into());
                return 1.into();
            }
        }
    }
    else {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if !fighter.sub_air_check_fall_common().get_bool() {
                if MotionModule::is_end(fighter.module_accessor) {
                    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    }
                    else {
                        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    }
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("ike")
    .status(Main, *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END, status_ike_SpecialSEnd_Main)
    .install();
}