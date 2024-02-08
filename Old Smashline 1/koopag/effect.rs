use crate::imports::BuildImports::*;

#[acmd_script(//AttackLw3
    agent = "koopag", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_koopag_attacklw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_scratch"), Hash40::new("koopag_scratch"), Hash40::new("top"), 5, 10, 22, 1, 27, -175.100006, 5.70000005, true, *EF_FLIP_YZ);
	}
}

#[acmd_script(//AttackAirN
    agent = "koopag", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_koopag_attackairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	frame(fighter.lua_state_agent, 22.0);
	if is_excute(fighter) {
		EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
}

#[acmd_script(//AttackAirLw
    agent = "koopag", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_koopag_attackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Move
    agent = "koopag_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_koopag_breath_move(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		EFFECT_FOLLOW(fighter, Hash40::new("koopag_breath"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.7, false);
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_koopag_attacklw3,
        effect_koopag_attackairn,
        effect_koopag_attackairlw,
		effect_koopag_breath_move
    );
}
