use crate::imports::BuildImports::*;

#[acmd_script(//AttackAirF 
    agent = "daisy", 
    script = "expression_attackairf", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_daisy_attackairf(fighter: &mut L2CAgentBase) {
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
    smashline::install_acmd_scripts!(
        expression_daisy_attackairf
    );
}
