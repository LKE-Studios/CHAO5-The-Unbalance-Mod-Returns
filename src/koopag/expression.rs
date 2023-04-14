use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;
use smash::hash40;

#[acmd_script(//AttackAirN
	agent = "koopag", 
	script = "expression_attackairn", 
	category = ACMD_EXPRESSION, 
	low_priority)]
unsafe fn expression_koopag_attackairn(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if macros::is_excute(fighter) {
		VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_shell") as i64);
	}
	frame(fighter.lua_state_agent, 36.0);
	if macros::is_excute(fighter) {
		VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
	}
}

#[acmd_script(//AttackAirLw
	agent = "koopag", 
	script = "expression_attackairlw", 
	category = ACMD_EXPRESSION, 
	low_priority)]
unsafe fn expression_koopag_attackairlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 13.0);
	if macros::is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), true);
	}
	frame(fighter.lua_state_agent, 53.0);
	if macros::is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), false);
	}
}

#[acmd_script(//LandingAirLw
	agent = "koopag", 
	script = "expression_landingairlw", 
	category = ACMD_EXPRESSION, 
	low_priority)]
unsafe fn expression_koopag_landingairlw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if macros::is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), false);
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
		expression_koopag_attackairn,
        expression_koopag_attackairlw,
		expression_koopag_landingairlw
    );
}