use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_KROOL )]
pub fn frame_krool(fighter : &mut L2CFighterCommon) {
    unsafe { 
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
            fighter.sub_air_check_fall_common();
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
            }
        }
        if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -40.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_krool
    );
}