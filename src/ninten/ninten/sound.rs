use crate::imports::BuildImports::*;

//Win3
unsafe extern "C" fn sound_ninten_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_ninten_win02"));
    }
}

//Wait2
unsafe extern "C" fn sound_ninten_Wait2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

//Wait3
unsafe extern "C" fn sound_ninten_Wait3(fighter: &mut L2CAgentBase) {}

//Attack11 
unsafe extern "C" fn sound_ninten_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_s"));
    }
}

//Attack12
unsafe extern "C" fn sound_ninten_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

//Attack13
unsafe extern "C" fn sound_ninten_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

//AttackDash
unsafe extern "C" fn sound_ninten_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_ninten_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_ninten_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_ninten_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_ninten_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_ninten_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal02"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("vc_ness_attack02"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_ninten_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_ninten_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_ninten_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_s"));
    }
}

//AttackAirF 
unsafe extern "C" fn sound_ninten_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_smash_s02"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_ninten_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_ninten_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_ninten_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    } 
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_swing_m"));
    }
}

//ThrowF
unsafe extern "C" fn sound_ninten_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

//ThrowB
unsafe extern "C" fn sound_ninten_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_ninten_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk02"));
    }
}      

//ThrowLw
unsafe extern "C" fn sound_ninten_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_l_01"));
    }
}

//SpecialHiStart
unsafe extern "C" fn sound_ninten_SpecialHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 144.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_ninten_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 144.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn sound_ninten_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
}

//SpecialAirHiAttack
unsafe extern "C" fn sound_ninten_SpecialAirHiAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_gohoubi_search_ui"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
    }
}

//SpecialLwHit
unsafe extern "C" fn sound_ninten_SpecialLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
}

//SpecialAirLwHit
unsafe extern "C" fn sound_ninten_SpecialAirLwHit(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_ninten_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_ninten_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
}

//AppealSR
unsafe extern "C" fn sound_ninten_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

//AppealSL
unsafe extern "C" fn sound_ninten_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_ninten_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_ninten_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
}

//Final
unsafe extern "C" fn sound_ninten_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        SoundModule::set_gamespeed_se_calibration(fighter.module_accessor, true);
        PLAY_STATUS(fighter, Hash40::new("se_ness_final01"));
        PLAY_STATUS(fighter, Hash40::new("vc_ness_final01"));
    }
    frame(fighter.lua_state_agent, 130.0);
    if is_excute(fighter) {
        SoundModule::set_gamespeed_se_calibration(fighter.module_accessor, false);
        PLAY_STATUS(fighter, Hash40::new("vc_ness_final02"));
    }
}

//FinalAir
unsafe extern "C" fn sound_ninten_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        SoundModule::set_gamespeed_se_calibration(fighter.module_accessor, true);
        PLAY_STATUS(fighter, Hash40::new("se_ness_final01"));
        PLAY_STATUS(fighter, Hash40::new("vc_ness_final01"));
    }
    frame(fighter.lua_state_agent, 130.0);
    if is_excute(fighter) {
        SoundModule::set_gamespeed_se_calibration(fighter.module_accessor, false);
        PLAY_STATUS(fighter, Hash40::new("vc_ness_final02"));
    }
}

pub fn install() {
    Agent::new("ness")
    .sound_acmd("sound_win3_ninten", sound_ninten_Win3, Low)
    .sound_acmd("sound_wait2_ninten", sound_ninten_Wait2, Low)
    .sound_acmd("sound_wait3_ninten", sound_ninten_Wait3, Low)
    .sound_acmd("sound_attack11_ninten", sound_ninten_Attack11, Low)
    .sound_acmd("sound_attack12_ninten", sound_ninten_Attack12, Low)
    .sound_acmd("sound_attack13_ninten", sound_ninten_Attack13, Low)
    .sound_acmd("sound_attackdash_ninten", sound_ninten_AttackDash, Low)
    .sound_acmd("sound_attackhi3_ninten", sound_ninten_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_ninten", sound_ninten_AttackLw3, Low)
    .sound_acmd("sound_attacks4_ninten", sound_ninten_AttackS4, Low)
    .sound_acmd("sound_attackhi4charge_ninten", sound_ninten_AttackHi4Charge, Low)
    .sound_acmd("sound_attackhi4_ninten", sound_ninten_AttackHi4, Low)
    .sound_acmd("sound_attacklw4charge_ninten", sound_ninten_AttackLw4Charge, Low)
    .sound_acmd("sound_attacklw4_ninten", sound_ninten_AttackLw4, Low)
    .sound_acmd("sound_attackairn_ninten", sound_ninten_AttackAirN, Low)
    .sound_acmd("sound_attackairf_ninten", sound_ninten_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_ninten", sound_ninten_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_ninten", sound_ninten_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_ninten", sound_ninten_AttackAirLw, Low)
    .sound_acmd("sound_throwf_ninten", sound_ninten_ThrowF, Low)
    .sound_acmd("sound_throwb_ninten", sound_ninten_ThrowB, Low)
    .sound_acmd("sound_throwhi_ninten", sound_ninten_ThrowHi, Low)
    .sound_acmd("sound_throwlw_ninten", sound_ninten_ThrowLw, Low)
    .sound_acmd("sound_specialhistart_ninten", sound_ninten_SpecialHiStart, Low)
    .sound_acmd("sound_specialairhistart_ninten", sound_ninten_SpecialAirHiStart, Low)
    .sound_acmd("sound_specialairhiend_ninten", sound_ninten_SpecialAirHiEnd, Low)
    .sound_acmd("sound_specialairhiattack_ninten", sound_ninten_SpecialAirHiAttack, Low)
    .sound_acmd("sound_speciallwhit_ninten", sound_ninten_SpecialLwHit, Low)
    .sound_acmd("sound_specialairlwhit_ninten", sound_ninten_SpecialAirLwHit, Low)
    .sound_acmd("sound_appealsr_ninten", sound_ninten_AppealSR, Low)
    .sound_acmd("sound_appealsl_ninten", sound_ninten_AppealSL, Low)
    .sound_acmd("sound_appealhir_ninten", sound_ninten_AppealHiR, Low)
    .sound_acmd("sound_appealhil_ninten", sound_ninten_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_ninten", sound_ninten_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_ninten", sound_ninten_AppealLwL, Low)
    .sound_acmd("sound_final_ninten", sound_ninten_Final, Low)
    .sound_acmd("sound_finalair_ninten", sound_ninten_FinalAir, Low)
    .install();
}
