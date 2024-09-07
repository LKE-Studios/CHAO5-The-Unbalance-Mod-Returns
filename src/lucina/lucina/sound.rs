use crate::imports::BuildImports::*;

//Attack13
unsafe extern "C" fn sound_lucina_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_lucina_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucina_attackl_s01"));
    }
}

//AttackS3S
unsafe extern "C" fn sound_lucina_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucina_attackl_s01"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_lucina_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucina_attackl_s01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_lucina_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucina_attack05"));
        PLAY_SE(fighter, Hash40::new("se_lucina_smash_s01"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_lucina_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
        PLAY_SE(fighter, Hash40::new("vc_lucina_attack06"));
        PLAY_SE(fighter, Hash40::new("se_lucina_smash_h01"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_lucina_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucina_attack07"));
        PLAY_SE(fighter, Hash40::new("se_lucina_smash_l01"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_smash_l01"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_lucina_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_lucina_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
}

//SpecialLwHit2
unsafe extern "C" fn sound_lucina_SpecialLwHit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_special_l02"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_special_l03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_special_l"));
    }
}

//SpecialAirLwHit2
unsafe extern "C" fn sound_lucina_SpecialAirLwHit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_special_l02"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucina_special_l03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_special_l"));
    }
}

pub fn install() {
    Agent::new("lucina")
    .sound_acmd("sound_attack13", sound_lucina_Attack13, Low)
    .sound_acmd("sound_attacks3hi", sound_lucina_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3", sound_lucina_AttackS3, Low)
    .sound_acmd("sound_attacks3lw", sound_lucina_AttackS3Lw, Low)
    .sound_acmd("sound_attacks4", sound_lucina_AttackS4, Low)
    .sound_acmd("sound_attackhi4", sound_lucina_AttackHi4, Low)
    .sound_acmd("sound_attacklw4", sound_lucina_AttackLw4, Low)
    .sound_acmd("sound_attackairn", sound_lucina_AttackAirN, Low)
    .sound_acmd("sound_attackairb", sound_lucina_AttackAirB, Low)
    .sound_acmd("sound_speciallwhit2", sound_lucina_SpecialLwHit2, Low)
    .sound_acmd("sound_specialairlwhit2", sound_lucina_SpecialAirLwHit2, Low)
    .install();
}