use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_MURABITO )]
pub fn frame_murabito(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_FALL {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
        if status_kind == *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_JUMP {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    };
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_murabito
    );
}