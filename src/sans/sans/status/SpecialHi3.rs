use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_SpecialHi3_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if SANS {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("rockman_hardknuckle"), false, false);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3)(fighter)
    } 
}

pub fn install() {
    Agent::new("palutena")
    .status(End, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3, status_sans_SpecialHi3_End)
    .install();
}