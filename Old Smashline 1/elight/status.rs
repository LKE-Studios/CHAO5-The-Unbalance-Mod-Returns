use crate::imports::BuildImports::*;

#[status_script(agent = "elight", status = FIGHTER_ELIGHT_STATUS_KIND_SPECIAL_S_END, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_elight_special_s_end_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_change_motion_by_situation(Hash40::new("special_s_end").into(), Hash40::new("special_air_s_end").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());
    fighter.sub_set_special_start_common_kinetic_setting(hash40("param_special_s").into());
    fighter.sub_shift_status_main(L2CValue::Ptr(elight_special_s_end_loop as *const () as _))
}

unsafe extern "C" fn elight_special_s_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let prev_situation_kind = fighter.global_table[PREV_SITUATION_KIND].get_i32();
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
                let frame = MotionModule::frame(fighter.module_accessor);
                let cancel_frame = FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new("special_s_end"), true);
                if frame >= cancel_frame {
                    fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
                    return 0.into()
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
    install_status_scripts!(
        status_elight_special_s_end_main
    );
}