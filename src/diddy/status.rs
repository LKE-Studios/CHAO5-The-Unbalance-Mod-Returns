use crate::imports::BuildImports::*;

#[status_script(agent = "diddy", status = FIGHTER_DIDDY_STATUS_KIND_SPECIAL_LW_LAUGH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_main_diddy_speciallwlaugh(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    if situation_kind == *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_lw_laugh"), 0.0, 1.0, false, 0.0, false, false);
    }
    if situation_kind == *SITUATION_KIND_AIR {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_lw_laugh"), 0.0, 1.0, false, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(status_main_diddy_speciallwlaugh_loop as *const () as _));
    0.into()
}

pub unsafe extern "C" fn status_main_diddy_speciallwlaugh_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);

    fighter.sub_wait_ground_check_common(false.into());
    fighter.sub_air_check_fall_common();
    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    if situation_kind == *SITUATION_KIND_GROUND {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD, false);
        }
    }
    if situation_kind == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
    }
    if [hash40("special_lw_laugh"), hash40("special_air_lw_laugh")].contains(&motion_kind) && MotionModule::is_end(fighter.module_accessor) {
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if situation_kind == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
    }
    0.into()
}

pub fn install() {
    smashline::install_status_scripts!(
        status_main_diddy_speciallwlaugh
    );
}