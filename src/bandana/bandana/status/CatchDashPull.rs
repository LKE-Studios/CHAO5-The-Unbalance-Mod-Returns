use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_CatchDashPull_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        fighter.status_CatchDashPull()
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, status_bandana_CatchDashPull_Main)
    .install();
}