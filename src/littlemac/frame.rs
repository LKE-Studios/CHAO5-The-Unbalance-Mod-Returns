use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_LITTLEMAC )]
pub fn frame_littlemac(fighter : &mut L2CFighterCommon) {
    unsafe {        
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_littlemac
    );
}