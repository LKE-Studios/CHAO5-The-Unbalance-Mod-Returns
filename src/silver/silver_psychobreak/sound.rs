use crate::imports::BuildImports::*;

///Hit 
unsafe extern "C" fn sound_silver_psychobreak_Hit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_ll"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_final03"));
    }
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_final03_02"));
    }
}

pub fn install() {
    Agent::new("mewtwo_psychobreak")
    .sound_acmd("sound_hit_silver", sound_silver_psychobreak_Hit, Low)
    .install();
}