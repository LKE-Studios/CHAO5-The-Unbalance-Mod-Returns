use crate::imports::BuildImports::*;

//Attack12
unsafe extern "C" fn sound_sonic_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
    }
}

//Attack13
unsafe extern "C" fn sound_sonic_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackDash
unsafe extern "C" fn sound_sonic_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_right_m"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_left_m"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_sonic_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_smash_h01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_sonic_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack07"));
        PLAY_STATUS(fighter, Hash40::new("se_sonic_smash_l01"));
    }
    wait(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

pub fn install() {
    Agent::new("sonic")
    .sound_acmd("sound_attack11_knuckles", sound_knuckles_Attack11, Low)
    .sound_acmd("sound_attack12_knuckles", sound_knuckles_Attack12, Low)
    .sound_acmd("sound_attack13_knuckles", sound_knuckles_Attack13, Low)
    .sound_acmd("sound_attack100_knuckles", sound_knuckles_Attack100, Low)
    .sound_acmd("sound_attack100sub_knuckles", sound_knuckles_Attack100Sub, Low)
    .sound_acmd("sound_attack100end_knuckles", sound_knuckles_Attack100End, Low)
    .sound_acmd("sound_attackdash_knuckles", sound_knuckles_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_knuckles", sound_knuckles_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_knuckles", sound_knuckles_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_knuckles", sound_knuckles_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_knuckles", sound_knuckles_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_knuckles", sound_knuckles_AttackLw3, Low)
    .sound_acmd("sound_attacks4hi_knuckles", sound_knuckles_AttackS4Hi, Low)
    .sound_acmd("sound_attacks4_knuckles", sound_knuckles_AttackS4, Low)
    .sound_acmd("sound_attacks4lw_knuckles", sound_knuckles_AttackS4Lw, Low)
    .sound_acmd("sound_attackhi4_knuckles", sound_knuckles_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_knuckles", sound_knuckles_AttackLw4, Low)
    .sound_acmd("sound_attackairn_knuckles", sound_knuckles_AttackAirN, Low)
    .sound_acmd("sound_attackairf_knuckles", sound_knuckles_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_knuckles", sound_knuckles_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_knuckles", sound_knuckles_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_knuckles", sound_knuckles_AttackAirLw, Low)
    .sound_acmd("sound_catch_knuckles", sound_knuckles_Catch, Low)
    .sound_acmd("sound_catchdash_knuckles", sound_knuckles_CatchDash, Low)
    .sound_acmd("sound_catchturn_knuckles", sound_knuckles_CatchTurn, Low)
    .sound_acmd("sound_catchattack_knuckles", sound_knuckles_CatchAttack, Low)
    .sound_acmd("sound_throwf_knuckles", sound_knuckles_ThrowF, Low)
    .sound_acmd("sound_throwb_knuckles", sound_knuckles_ThrowB, Low)
    .sound_acmd("sound_throwhi_knuckles", sound_knuckles_ThrowHi, Low)
    .sound_acmd("sound_throwlw_knuckles", sound_knuckles_ThrowLw, Low)
    .sound_acmd("sound_downattackd_knuckles", sound_knuckles_DownAttackD, Low)
    .sound_acmd("sound_downattacku_knuckles", sound_knuckles_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_knuckles", sound_knuckles_CliffAttack, Low)
    .sound_acmd("sound_slipattack_knuckles", sound_knuckles_SlipAttack, Low)
    .sound_acmd("sound_specialndig_knuckles", sound_knuckles_SpecialNDig, Low)
    .sound_acmd("sound_specialairndig_knuckles", sound_knuckles_SpecialAirNDig, Low)
    .sound_acmd("sound_specialairndrillloop_knuckles", sound_knuckles_SpecialAirNDrillLoop, Low)
    .sound_acmd("sound_landingfallspecial_knuckles", sound_knuckles_LandingFallSpecial, Low)
    .sound_acmd("sound_specialnupper_knuckles", sound_knuckles_SpecialNUpper, Low)  
    .sound_acmd("sound_specials_knuckles", sound_knuckles_SpecialS, Low) 
    .sound_acmd("sound_specialairsturn_knuckles", sound_knuckles_SpecialAirSTurn, Low)
    .sound_acmd("sound_specialairsendloop_knuckles", sound_knuckles_SpecialAirSEndLoop, Low)
    .sound_acmd("sound_specialhi_knuckles", sound_knuckles_SpecialHi, Low)
    .sound_acmd("sound_speciallwhold_knuckles", sound_knuckles_SpecialLwHold, Low)
    .sound_acmd("0x195dc47911", sound_knuckles_0x195dc47911, Low)
    .sound_acmd("sound_appealsr_knuckles", sound_knuckles_AppealSR, Low)
    .sound_acmd("sound_appealsl_knuckles", sound_knuckles_AppealSL, Low)
    .sound_acmd("sound_appealhir_knuckles", sound_knuckles_AppealHiR, Low)
    .sound_acmd("sound_appealhil_knuckles", sound_knuckles_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_knuckles", sound_knuckles_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_knuckles", sound_knuckles_AppealLwL, Low)
    .install();
}