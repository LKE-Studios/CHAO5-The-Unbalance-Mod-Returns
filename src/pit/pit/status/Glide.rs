use crate::imports::BuildImports::*;

unsafe extern "C" fn status_pit_Glide_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 0);
    fighter.status_end_Glide()
}

pub fn install() {
    Agent::new("pit")
    .status(End, *FIGHTER_STATUS_KIND_GLIDE, status_pit_Glide_End)
    .install();
}