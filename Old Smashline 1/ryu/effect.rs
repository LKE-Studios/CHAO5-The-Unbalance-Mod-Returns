use crate::imports::BuildImports::*;

#[acmd_script(
    agent = "ryu",
    script = "effect_kamehameha_fire",
    category = ACMD_EFFECT,
low_priority )]
unsafe fn effect_ryu_kamehameha_fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("top"), 10, 9, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_genesis_beam"), true, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_ryu_kamehameha_fire,
        //effect_ryu_kamehameha_charge,
        //effect_ryu_kamehameha_start
    );
}