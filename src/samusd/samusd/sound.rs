use crate::imports::BuildImports::*;

//AttackS3Hi
unsafe extern "C" fn sound_samusd_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_special_n03"));
    }
}

//AttackS3
unsafe extern "C" fn sound_samusd_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_special_n03"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_samusd_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_special_n03"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_samusd_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_samusd_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_samusd_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_samusd_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

//AttackAirLw 
unsafe extern "C" fn sound_samusd_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

pub fn install() {
    Agent::new("samusd")
    .sound_acmd("sound_attacks3hi", sound_samusd_AttackS3Hi)
    .sound_acmd("sound_attacks3", sound_samusd_AttackS3)
    .sound_acmd("sound_attacks3lw", sound_samusd_AttackS3Lw)
    .sound_acmd("sound_attacklw4", sound_samusd_AttackLw4)
    .sound_acmd("sound_attackairf", sound_samusd_AttackAirF)
    .sound_acmd("sound_attackairb", sound_samusd_AttackAirB)
    .sound_acmd("sound_attackairhi", sound_samusd_AttackAirHi)
    .sound_acmd("sound_attackairlw", sound_samusd_AttackAirLw)
    .install();
}