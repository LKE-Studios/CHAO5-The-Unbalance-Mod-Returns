use crate::imports::BuildImports::*;

//Regular
unsafe extern "C" fn effect_waluigi_diceblock_Regular(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.8, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.4, /*G*/ 0.0, /*B*/ 2.0);
	}
}

//Bound
unsafe extern "C" fn effect_waluigi_diceblock_Bound(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, -4, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
	}
}

//Break
unsafe extern "C" fn effect_waluigi_diceblock_Break(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, -4, 1, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
		EFFECT(fighter, Hash40::new("sys_assist"), Hash40::new("top"), 0, 4, -1, 0, 0, 180, 0.6, 0, 0, 0, 0, 0, 0, false);
	}
}

pub fn install() {
    Agent::new("dolly_diceblock")
    .effect_acmd("effect_regular", effect_waluigi_diceblock_Regular)
    .effect_acmd("effect_bound", effect_waluigi_diceblock_Bound)
    .effect_acmd("effect_break", effect_waluigi_diceblock_Break)
    .install();
}