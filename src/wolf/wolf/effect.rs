use crate::imports::BuildImports::*;

//SpecialS
unsafe extern "C" fn effect_wolf_SpecialS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash"), Hash40::new("top"), -3, 5.5, 0, 65, 0, 0, 0.75, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_rush"), Hash40::new("top"), 4, 20.7, 35, 65, 0, 0, 1, false);
    }
}

//SpecialAirS
unsafe extern "C" fn effect_wolf_SpecialAirS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash"), Hash40::new("top"), -3, 5.5, 0, 65, 0, 0, 0.75, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_rush"), Hash40::new("top"), 4, 20.7, 35, 65, 0, 0, 1, false);
    }
}

//SpecialSEnd
unsafe extern "C" fn effect_wolf_SpecialSEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_scratch"), Hash40::new("top"), 5, 13, 0, -40, 0, 0, 0.45, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5.5, 5.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

//SpecialAirSEnd
unsafe extern "C" fn effect_wolf_SpecialAirSEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("wolf_slash_scratch"), Hash40::new("top"), 5, 13, 0, -40, 0, 0, 0.45, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 5.5, 5.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

pub fn install() {
    Agent::new("wolf")
    .effect_acmd("effect_specials", effect_wolf_SpecialS, Low)
    .effect_acmd("effect_specialairs", effect_wolf_SpecialAirS, Low)
    .effect_acmd("effect_specialsend", effect_wolf_SpecialSEnd, Low)
    .effect_acmd("effect_specialairsend", effect_wolf_SpecialAirSEnd, Low)
    .install();
}