use crate::imports::BuildImports::*;

//AttackAirF 
unsafe extern "C" fn expression_daisy_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

pub fn install() {
    Agent::new("daisy")
    .expression_acmd("expression_attackairf", expression_daisy_AttackAirF)
    .install();
}
