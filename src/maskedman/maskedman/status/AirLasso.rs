use crate::imports::BuildImports::*;

unsafe extern "C" fn status_maskedman_AirLasso_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let MASKEDMAN = color >= 120 && color <= 127;
	if MASKEDMAN {
        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), false.into());
        0.into()
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_AIR_LASSO)(fighter)
    }	
}

pub fn install() {
    Agent::new("lucas")
    .status(Main, *FIGHTER_STATUS_KIND_AIR_LASSO, status_maskedman_AirLasso_Main)
    .install();
}