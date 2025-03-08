use crate::imports::BuildImports::*;

//AttackLw3
unsafe extern "C" fn effect_koopag_AttackLw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_scratch"), Hash40::new("koopag_scratch"), Hash40::new("top"), 5, 10, 22, 1, 27, -175.100006, 5.70000005, true, *EF_FLIP_YZ);
	}
}

//AttackAirN
unsafe extern "C" fn effect_koopag_AttackAirN(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 0, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	frame(fighter.lua_state_agent, 22.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 0, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
}

//AttackAirLw
unsafe extern "C" fn effect_koopag_AttackAirLw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 15.0);
	for _ in 0..7 {
        if is_excute(fighter) {
			EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
		}
		wait(fighter.lua_state_agent, 3.0);
	}
	frame(fighter.lua_state_agent, 41.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
	}
}

//ShieldBreakFly
unsafe extern "C" fn effect_koopag_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FLW_POS(fighter, Hash40::new("sys_piyopiyo"), Hash40::new("head"), 6, 4, 0, 0, 0, 0, 1.4, true);
    }
}

//FuraFura
unsafe extern "C" fn effect_koopag_FuraFuraEnd(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_piyo"), Hash40::new("top"), 0, 12, 16, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
    }
}

pub fn install() {
    Agent::new("koopag")
	.effect_acmd("effect_attack13", effect_koopag_AttackLw3, Low)
    .effect_acmd("effect_attackairn", effect_koopag_AttackAirN, Low)
    .effect_acmd("effect_attackairlw", effect_koopag_AttackAirLw, Low)
	.effect_acmd("effect_furafuraend", effect_koopag_FuraFuraEnd, Low)
	.effect_acmd("effect_shieldbreakfly", effect_koopag_ShieldBreakFly, Low)
    .install();
}