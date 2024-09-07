use crate::imports::BuildImports::*;

//AttackDash 
unsafe extern "C" fn sound_younglink_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l02"));
        PLAY_SE(fighter, Hash40::new("se_younglink_escape"))
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_landing01"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_younglink_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_younglink_attack04"));
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_ll"));
    }
    wait(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_ll"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_younglink_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l"));
    }
}

pub fn install() {
    Agent::new("younglink")
    .sound_acmd("sound_attackdash", sound_younglink_AttackDash, Low)
    .sound_acmd("sound_attackhi4", sound_younglink_AttackHi4, Low)
    .sound_acmd("sound_attackairf", sound_younglink_AttackAirF, Low)    
    .install();
}