use crate::imports::BuildImports::*;

//Attack11 
unsafe extern "C" fn sound_funky_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_s01"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_donkey_smash_s01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_clap"));
    }
}

//AttackDash
unsafe extern "C" fn sound_funky_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackdash"));
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_funky_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_s01"));
    }
}

//AttackS3
unsafe extern "C" fn sound_funky_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_s01"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_funky_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_s01"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_funky_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_h01"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_donkey_smash_h01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_clap"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_funky_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_l01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_funky_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack06"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_h01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_funky_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_landing02"));
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack05"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_attackair_n01"));
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_landing01"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_funky_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack07"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_l01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_l02"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_l01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_l02"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_funky_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h02"));
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_funky_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_attackair_f01"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_funky_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_attackhard_h01"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_funky_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h06"));
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_funky_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h06"));
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//ThrowF
unsafe extern "C" fn sound_funky_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowB
unsafe extern "C" fn sound_funky_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_funky_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowLw
unsafe extern "C" fn sound_funky_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_funky_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_jump02"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_l"));
    }
    wait(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_smash_l02"));
    }
}

//SpecialN
unsafe extern "C" fn sound_funky_SpecialN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_donkey_special_n06"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack06"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n01"));
    }
}

//SpecialNLoop
unsafe extern "C" fn sound_funky_SpecialNLoop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n06"));
    }
}

//SpecialNMax
unsafe extern "C" fn sound_funky_SpecialNMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_001"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_lifeup"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n02"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n05"));
    }
}

//SpecialAirN
unsafe extern "C" fn sound_funky_SpecialAirN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_donkey_special_n06"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack06"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n01"));
    }
}

//SpecialAirNLoop
unsafe extern "C" fn sound_funky_SpecialAirNLoop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n06"));
    }
}

//SpecialAirNMax
unsafe extern "C" fn sound_funky_SpecialAirNMax(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_001"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_lifeup"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n02"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n05"));
    }
}

//SpecialSStart
unsafe extern "C" fn sound_funky_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_final05"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_s02"));
    }
}

//SpecialAirSStart
unsafe extern "C" fn sound_funky_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_final05"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_s02"));
    }
}

//SpecialSLanding
unsafe extern "C" fn sound_funky_SpecialSLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_s02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_n03"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
    }
}

//SpecialAirSDirection
unsafe extern "C" fn sound_funky_SpecialAirSDirection(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_l02"));
    }
}

//SpecialAirSEnd
unsafe extern "C" fn sound_funky_SpecialAirSEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
    }
}

//SpecialHiEnd
unsafe extern "C" fn sound_funky_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_donkey_landing03"));
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_funky_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_l01"));
    }
}

//SpecialAirHiLaunch
unsafe extern "C" fn sound_funky_SpecialAirHiLaunch(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appear01"));
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h09"));
    }
}

//SpecialHi_C2
unsafe extern "C" fn sound_funky_SpecialHi_C2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h01_01"));
    }
}

//SpecialAirHi_C2
unsafe extern "C" fn sound_funky_SpecialAirHi_C2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_donkey_special_h01_02"));
    }
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h01_03"));
    }
}

//SpecialLwStart
unsafe extern "C" fn sound_funky_SpecialLwStart(fighter: &mut L2CAgentBase) {
    let rand_val = sv_math::rand(hash40("fighter"), 4);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        if rand_val == 1 {
            PLAY_SE(fighter, Hash40::new("vc_donkey_win01_02"));
        }
        else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h05"));
    }
}

//SpecialLwJump
unsafe extern "C" fn sound_funky_SpecialLwJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_jump02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//SpecialLwMusic
unsafe extern "C" fn sound_funky_SpecialLwMusic(fighter: &mut L2CAgentBase) {
    let rand_val = sv_math::rand(hash40("donkey"), 2);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        if rand_val == 0 {
            PLAY_SE(fighter, Hash40::new("se_donkey_special_s01"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_donkey_win2_01"));
        }
    }
}

//SpecialLwPose
unsafe extern "C" fn sound_funky_SpecialLwPose(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_m"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//SpecialLwFlip
unsafe extern "C" fn sound_funky_SpecialLwFlip(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_jump02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_funky_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    let rand_val = sv_math::rand(hash40("fighter"), 4);
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        if rand_val == 1 {
            PLAY_SE(fighter, Hash40::new("vc_donkey_win01_02"));
        }
        else {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
        }
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h05"));
    }
}

//SpecialAirLwJump
unsafe extern "C" fn sound_funky_SpecialAirLwJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_jump02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//SpecialAirLwMusic
unsafe extern "C" fn sound_funky_SpecialAirLwMusic(fighter: &mut L2CAgentBase) {
    let rand_val = sv_math::rand(hash40("donkey"), 2);
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        if rand_val == 0 {
            PLAY_SE(fighter, Hash40::new("se_donkey_special_s01"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_donkey_win2_01"));
        }
    }
}

//SpecialAirLwPose
unsafe extern "C" fn sound_funky_SpecialAirLwPose(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_swing_m"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//SpecialAirLwFlip
unsafe extern "C" fn sound_funky_SpecialAirLwFlip(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_jump02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_donkey_rnd_attack"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_funky_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_rise"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_landing01"));
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack01"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_funky_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_rise"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_landing01"));
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack01"));
    }
}

//AppealSR
unsafe extern "C" fn sound_funky_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 74.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_001"));
    }
}

//AppealSL
unsafe extern "C" fn sound_funky_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_appeal_h01"));
    }
    frame(fighter.lua_state_agent, 74.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_001"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_funky_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack03"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_rise"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_funky_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack03"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_rise"));
    }
}

//FinalStart
unsafe extern "C" fn sound_funky_FinalStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_missfoot02"));
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h04"));
    }
}

//FinalAirStart
unsafe extern "C" fn sound_funky_FinalAirStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_missfoot02"));
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h04"));
    }
}

//Win1
unsafe extern "C" fn sound_funky_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_attack01"));
    }
    frame(fighter.lua_state_agent, 82.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_donkey_cliffcatch"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_funky_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_funky_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_funky_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_funky_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_funky_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_donkey_rnd_futtobi01"), Hash40::new("seq_donkey_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("donkey")
    .sound_acmd("sound_attack11_funky", sound_funky_Attack11, Low)
    .sound_acmd("sound_attackdash_funky", sound_funky_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_funky", sound_funky_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_funky", sound_funky_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_funky", sound_funky_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_funky", sound_funky_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_funky", sound_funky_AttackLw3, Low)
    .sound_acmd("sound_attacks4_funky", sound_funky_AttackS4, Low)
    .sound_acmd("sound_attackhi4_funky", sound_funky_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_funky", sound_funky_AttackLw4, Low)
    .sound_acmd("sound_attackairn_funky", sound_funky_AttackAirN, Low)
    .sound_acmd("sound_attackairf_funky", sound_funky_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_funky", sound_funky_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_funky", sound_funky_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_funky", sound_funky_AttackAirLw, Low)
    .sound_acmd("sound_throwf_funky", sound_funky_ThrowF, Low)
    .sound_acmd("sound_throwb_funky", sound_funky_ThrowB, Low)
    .sound_acmd("sound_throwhi_funky", sound_funky_ThrowHi, Low)
    .sound_acmd("sound_throwlw_funky", sound_funky_ThrowLw, Low)
    .sound_acmd("sound_cliffattack_funky", sound_funky_CliffAttack, Low)
    .sound_acmd("sound_specialn_funky", sound_funky_SpecialN, Low)
    .sound_acmd("sound_specialnloop_funky", sound_funky_SpecialNLoop, Low)
    .sound_acmd("sound_specialnmax_funky", sound_funky_SpecialNMax, Low)
    .sound_acmd("sound_specialairn_funky", sound_funky_SpecialAirN, Low)
    .sound_acmd("sound_specialairnloop_funky", sound_funky_SpecialAirNLoop, Low)
    .sound_acmd("sound_specialairnmax_funky", sound_funky_SpecialAirNMax, Low)
    .sound_acmd("sound_specialsstart_funky", sound_funky_SpecialSStart, Low)
    .sound_acmd("sound_specialslanding_funky", sound_funky_SpecialSLanding, Low)
    .sound_acmd("sound_specialairsstart_funky", sound_funky_SpecialAirSStart, Low)
    .sound_acmd("sound_specialairsdirection_funky", sound_funky_SpecialAirSDirection, Low)
    .sound_acmd("sound_specialairsend_funky", sound_funky_SpecialAirSEnd, Low)
    .sound_acmd("sound_specialairhi_funky", sound_funky_SpecialAirHi, Low)
    .sound_acmd("sound_specialairhilaunch_funky", sound_funky_SpecialAirHiLaunch, Low)
    .sound_acmd("sound_specialhic2_funky", sound_funky_SpecialHi_C2, Low)
    .sound_acmd("sound_specialairhic2_funky", sound_funky_SpecialAirHi_C2, Low)
    .sound_acmd("sound_speciallwstart_funky", sound_funky_SpecialLwStart, Low)
    .sound_acmd("sound_speciallwjump_funky", sound_funky_SpecialLwJump, Low)
    .sound_acmd("sound_speciallwmusic_funky", sound_funky_SpecialLwMusic, Low)
    .sound_acmd("sound_speciallwflip_funky", sound_funky_SpecialLwFlip, Low)
    .sound_acmd("sound_speciallwpose_funky", sound_funky_SpecialLwPose, Low)
    .sound_acmd("sound_specialairlwstart_funky", sound_funky_SpecialAirLwStart, Low)
    .sound_acmd("sound_specialairlwjump_funky", sound_funky_SpecialAirLwJump, Low)
    .sound_acmd("sound_specialairlwmusic_funky", sound_funky_SpecialAirLwMusic, Low)
    .sound_acmd("sound_specialairlwflip_funky", sound_funky_SpecialAirLwFlip, Low)
    .sound_acmd("sound_specialairlwpose_funky", sound_funky_SpecialAirLwPose, Low)
    .sound_acmd("sound_appealhir_funky", sound_funky_AppealHiR, Low)
    .sound_acmd("sound_appealhil_funky", sound_funky_AppealHiL, Low)
    .sound_acmd("sound_appealsr_funky", sound_funky_AppealSR, Low)
    .sound_acmd("sound_appealsl_funky", sound_funky_AppealSL, Low)
    .sound_acmd("sound_appeallwr_funky", sound_funky_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_funky", sound_funky_AppealLwL, Low)
    .sound_acmd("sound_finalstart_funky", sound_funky_FinalStart, Low)
    .sound_acmd("sound_finalairstart_funky", sound_funky_FinalAirStart, Low)
    .sound_acmd("sound_win1_funky", sound_funky_Win1, Low)
    .sound_acmd("sound_damageflyhi_funky", sound_funky_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw_funky", sound_funky_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn_funky", sound_funky_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop_funky", sound_funky_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll_funky", sound_funky_DamageFlyRoll, Low)
    .install();
}