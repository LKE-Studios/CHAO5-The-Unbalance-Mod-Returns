use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_SpecialN_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(module_accessor, Hash40::new("se_edge_special_n02_03"), 0);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_N)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, status_bandana_SpecialN_End)
    .install();
}