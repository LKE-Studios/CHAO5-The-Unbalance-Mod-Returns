use crate::imports::BuildImports::*;

//Move
unsafe extern "C" fn effect_koopag_breath_Move(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("brave_fire3_bullet"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, false);
	}
}

pub fn install() {
    Agent::new("koopag_breath")
    .effect_acmd("effect_move", effect_koopag_breath_Move, Low)
    .install();
}