use crate::imports::BuildImports::*;

unsafe extern "C" fn status_koopag_AttackAir_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
	L2CFighterCommon_status_AttackAir(fighter)
}

pub fn install() {
    Agent::new("koopag")
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, status_koopag_AttackAir_Main)
    .install();
}