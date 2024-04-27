use crate::imports::BuildImports::*;

//SpecialS
unsafe extern "C" fn sound_wolf_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.33);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_wolf_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_wolf_special_s02"));
    }
}

//SpecialAirS
unsafe extern "C" fn sound_wolf_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.33);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_wolf_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_wolf_special_s02"));
    }
}

pub fn install() {
    Agent::new("wolf")
    .sound_acmd("sound_specials", sound_wolf_SpecialS, Low)
    .sound_acmd("sound_specialairs", sound_wolf_SpecialAirS, Low)
    .install();
}