use crate::imports::BuildImports::*;

unsafe extern "C" fn status_bandana_AttackAir_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("littlemac_risinguppercut"), false, false);
        0.into()
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_ATTACK_AIR)(fighter)
    }
}

pub fn install() {
    Agent::new("edge")
    .status(End, *FIGHTER_STATUS_KIND_ATTACK_AIR, status_bandana_AttackAir_End)
    .install();
}