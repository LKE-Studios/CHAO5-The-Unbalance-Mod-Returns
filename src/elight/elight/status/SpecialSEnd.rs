use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_elight_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(elight_SpecialSEnd_Main_loop as *const () as _))
}

unsafe extern "C" fn elight_SpecialSEnd_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
    let frame = MotionModule::frame(fighter.module_accessor);
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_transition_group_check_air_cliff().get_bool() {
            if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
                if fighter.sub_air_check_fall_common().get_bool() {
                    return 0.into()
                }
            }
        }
        if !MotionModule::is_end(fighter.module_accessor) {
            if situation_kind == *SITUATION_KIND_GROUND && prev_situation_kind != *SITUATION_KIND_GROUND {
                SoundModule::play_landing_se(fighter.module_accessor, Hash40::new("se_elight_landing01"));
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_end"), true);
                if frame >= cancel_frame {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                    return 0.into()
                }
            }
            if frame > 23.0 {
                if situation_kind != *SITUATION_KIND_GROUND {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                }
                else {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                }
            }
        }
    }    
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), true.into());
        fighter.sub_set_ground_correct_by_situation(true.into());
        fighter.sub_exec_special_start_common_kinetic_setting(hash40("param_special_s").into());
        if situation_kind != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("elight")
    .status(Main, *FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, status_elight_SpecialSEnd_Main)
    .install();
}