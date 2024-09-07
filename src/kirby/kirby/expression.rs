use crate::imports::BuildImports::*;

//SpecialHiH
unsafe extern "C" fn expression_kirby_SpecialHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

//SpecialAirHiH
unsafe extern "C" fn expression_kirby_SpecialAirHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("kirby")
    .expression_acmd("expression_specialhih", expression_kirby_SpecialHiH, Low)
    .expression_acmd("expression_specialairhih", expression_kirby_SpecialAirHiH, Low)
    .install();
}