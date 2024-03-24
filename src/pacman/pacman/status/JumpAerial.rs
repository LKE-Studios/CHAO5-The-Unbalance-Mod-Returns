use crate::imports::BuildImports::*;

unsafe extern "C" fn status_pacman_JumpAerial_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_JumpAerial()
}

pub fn install() {
    Agent::new("pacman")
    .status(Main, *FIGHTER_STATUS_KIND_JUMP_AERIAL, status_pacman_JumpAerial_Main)
    .install();
}