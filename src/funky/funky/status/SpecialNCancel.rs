use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_SpecialNCancel_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_donkey_special_n06"), 0);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(End, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL, status_funky_SpecialNCancel_End)    
    .install();
}