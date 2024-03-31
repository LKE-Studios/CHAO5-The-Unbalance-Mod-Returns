use crate::imports::BuildImports::*;

//SpecialZ
unsafe extern "C" fn sound_pzenigame_SpecialZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
        PLAY_SE(fighter, Hash40::new("se_pzenigame_swing_m"));
    }
}

//SpecialAirZ
unsafe extern "C" fn sound_pzenigame_SpecialAirZ(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pzenigame_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pzenigame_bubble"));
        PLAY_SE(fighter, Hash40::new("se_pzenigame_swing_m"));
    }
}

pub fn install() {
    Agent::new("pzenigame")
    .sound_acmd("sound_specialz", sound_pzenigame_SpecialZ)
    .sound_acmd("sound_specialairz", sound_pzenigame_SpecialAirZ)
    .install();
}