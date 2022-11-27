<<<<<<< HEAD
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;



#[acmd_script(//AttackLw3
    agent = "koopag", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attacklw3gfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_scratch"), Hash40::new("koopag_scratch"), Hash40::new("top"), 5, 10, 22, 1, 27, -175.100006, 5.70000005, true, *EF_FLIP_YZ);
	}
}

#[acmd_script(//AttackAirN
    agent = "koopag", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attackairngfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
}

#[acmd_script(//AttackAirLw
    agent = "koopag", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attackairlwgfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 15.0);
	for _ in 0..7 {
        if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
		}
		wait(fighter.lua_state_agent, 3.0);
	}
	frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
	}
}

#[acmd_script(//Move
    agent = "koopag_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_firegfx(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("koopag_breath"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.7, false);
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        koopag_attacklw3gfx,
        koopag_attackairngfx,
        koopag_attackairlwgfx,
		koopag_firegfx
    );
=======
use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;



#[acmd_script(//AttackLw3
    agent = "koopag", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attacklw3gfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 17.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_scratch"), Hash40::new("koopag_scratch"), Hash40::new("top"), 5, 10, 22, 1, 27, -175.100006, 5.70000005, true, *EF_FLIP_YZ);
	}
}

#[acmd_script(//AttackAirN
    agent = "koopag", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attackairngfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
	frame(fighter.lua_state_agent, 22.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -14, 5, 30, 0, 0, 4.45000005, true, *EF_FLIP_XY);
	}
}

#[acmd_script(//AttackAirLw
    agent = "koopag", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_attackairlwgfx(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 15.0);
	for _ in 0..7 {
        if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
		}
		wait(fighter.lua_state_agent, 3.0);
	}
	frame(fighter.lua_state_agent, 41.0);
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopag_shell_a"), Hash40::new("koopag_shell_b"), Hash40::new("rot"), 0, -24, 5, 0, 0, 0, 5.45000005, true, *EF_FLIP_XY);
	}
}

#[acmd_script(//Move
    agent = "koopag_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn koopag_firegfx(fighter: &mut L2CAgentBase) {
	if macros::is_excute(fighter) {
		macros::EFFECT_FOLLOW(fighter, Hash40::new("koopag_breath"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.7, false);
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        koopag_attacklw3gfx,
        koopag_attackairngfx,
        koopag_attackairlwgfx,
		koopag_firegfx
    );
>>>>>>> 70e591ed528511fa22d74147c05b77221fd7f3d5
}