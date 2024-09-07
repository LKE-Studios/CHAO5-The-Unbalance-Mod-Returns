use crate::imports::BuildImports::*;

unsafe extern "C" fn status_waluigi_LandingLight_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let WALUIGI = color >= 120 && color <= 130;
	if WALUIGI && fighter_kind == FIGHTER_KIND_DOLLY {	
        fighter.sub_landing_uniq_process_init()
    }
    else {
        original_status(Init, fighter, *FIGHTER_STATUS_KIND_LANDING_LIGHT)(fighter)
    }	
}

pub fn install() {
    Agent::new("dolly")
    .status(Init, *FIGHTER_STATUS_KIND_LANDING_LIGHT, status_waluigi_LandingLight_Init)
    .install();
}

