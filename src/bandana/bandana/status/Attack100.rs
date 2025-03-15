use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_Attack100_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_edge_attackair_f03_1"), 0);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_100)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_100, status_bandana_Attack100_End)
    .install();
}