use crate::imports::BuildImports::*;

///Hit 
unsafe extern "C" fn effect_silver_psychobreak_Hit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_attack"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.45, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mewtwo_final_hit"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
}

pub fn install() {
    Agent::new("mewtwo_psychobreak")
    .effect_acmd("effect_hit_silver", effect_silver_psychobreak_Hit, Low)
    .install();
}