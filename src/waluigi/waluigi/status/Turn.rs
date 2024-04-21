use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_Turn_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {			
        fighter.status_pre_Turn()
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_TURN)(fighter)
    }	
}

pub fn install() {
    Agent::new("dolly")
    .status(Pre, *FIGHTER_STATUS_KIND_TURN, status_waluigi_Turn_Pre)
    .install();
}

