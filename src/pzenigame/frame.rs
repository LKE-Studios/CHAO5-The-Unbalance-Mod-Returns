use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_PZENIGAME )]
fn frame_pzenigame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) && 
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_BLOW, false);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_pzenigame
    );
}