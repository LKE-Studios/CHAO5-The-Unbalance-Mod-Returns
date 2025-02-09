use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_funky_SpecialNLoop_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let FUNKY = color >= 120 && color <= 127;
	if FUNKY {
        FighterUtil::cancel_face_motion_by_priority(fighter.module_accessor, FighterFacial(*FIGHTER_FACIAL_SPECIAL));
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP)(fighter)
    }
}

pub fn install() {
    Agent::new("donkey")
    .status(End, *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP, status_funky_SpecialNLoop_End)    
    .install();
}