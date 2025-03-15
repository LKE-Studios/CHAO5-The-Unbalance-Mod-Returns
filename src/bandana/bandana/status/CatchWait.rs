use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_CatchWait_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        fighter.status_CatchWait()
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_WAIT)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, status_bandana_CatchWait_Main)
    .install();
}