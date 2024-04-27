use crate::imports::BuildImports::*;

//AttackAirN
unsafe extern "C" fn expression_koopag_AttackAirN(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_shell") as i64);
	}
	frame(fighter.lua_state_agent, 36.0);
	if is_excute(fighter) {
		VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
	}
}

//AttackAirLw
unsafe extern "C" fn expression_koopag_AttackAirLw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 13.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), true);
	}
	frame(fighter.lua_state_agent, 53.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), false);
	}
}

//LandingAirLw
unsafe extern "C" fn expression_koopag_LandingAirLw(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("shellmodel"), false);
	}
}

pub fn install() {
    Agent::new("koopag")
    .expression_acmd("expression_attackairn", expression_koopag_AttackAirN, Low)
    .expression_acmd("expression_attackairlw", expression_koopag_AttackAirLw, Low)
    .expression_acmd("expression_landingairlw", expression_koopag_LandingAirLw, Low)
    .install();
}