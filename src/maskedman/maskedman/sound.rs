use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn sound_maskedman_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    }
}

//AttackDash
unsafe extern "C" fn sound_maskedman_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackdash"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_maskedman_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
}

//AttackS3
unsafe extern "C" fn sound_maskedman_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_maskedman_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_maskedman_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_m"));
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_maskedman_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_s"));
    }
}

//AttackS4Charge
unsafe extern "C" fn sound_maskedman_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackS4
unsafe extern "C" fn sound_maskedman_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_offset"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_l"));
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_l"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_s02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_maskedman_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_maskedman_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_smash_l02"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_h01"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_h02"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_maskedman_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_maskedman_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_maskedman_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_l"));
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_l"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_maskedman_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f02"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f01"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_maskedman_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_b01"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_b02"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_b03"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_maskedman_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_m"));
        PLAY_SE(fighter, Hash40::new("se_common_sword_swing_m"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_maskedman_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f01"));
    }
}

//Catch
unsafe extern "C" fn sound_maskedman_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//CatchDash
unsafe extern "C" fn sound_maskedman_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//CatchTurn
unsafe extern "C" fn sound_maskedman_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//ThrowF
unsafe extern "C" fn sound_maskedman_ThrowF(fighter: &mut L2CAgentBase) {    
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_f01"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_f02"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowB
unsafe extern "C" fn sound_maskedman_ThrowB(fighter: &mut L2CAgentBase) {  
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_b01"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_b02"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_maskedman_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_h01"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_h02"));
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}      

//ThrowLw
unsafe extern "C" fn sound_maskedman_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_maskedman_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_dash_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
}

//SlipAttack
unsafe extern "C" fn sound_maskedman_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_rise"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_maskedman_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_dash_start"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_m"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
}

//DownAttackD
unsafe extern "C" fn sound_maskedman_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_rise"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_m"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_lucas_step_left_m"));
    }
}

//SpecialNStart
unsafe extern "C" fn sound_maskedman_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_n02"));
    }
}

//SpecialAirNStart
unsafe extern "C" fn sound_maskedman_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_n02"));
    }
}

//SpecialNDash
unsafe extern "C" fn sound_maskedman_SpecialNDash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_lucas_special_n02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_l"));
        PLAY_SE(fighter, Hash40::new("se_lucas_special_n05"));
    }
}

//SpecialAirNDash
unsafe extern "C" fn sound_maskedman_SpecialAirNDash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_lucas_special_n02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_beamsword_l"));
        PLAY_SE(fighter, Hash40::new("se_lucas_special_n05"));
    }
}

//SpecialS
unsafe extern "C" fn sound_maskedman_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
    }
}       

//SpecialAirS
unsafe extern "C" fn sound_maskedman_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
    }
}

//SpecialHiStart
unsafe extern "C" fn sound_maskedman_SpecialHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_h02"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_maskedman_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_h02"));
    }
}

//SpecialHiHold
unsafe extern "C" fn sound_maskedman_SpecialHiHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_lucas_special_h01"));
    }
}

//SpecialLwStart
unsafe extern "C" fn sound_maskedman_SpecialLwStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_l02"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_l02"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_maskedman_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_l02"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_throw_l02"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_maskedman_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_landing02"));
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f01"));
    }
    frame(fighter.lua_state_agent, 87.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appeal01_02"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_maskedman_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_landing02"));
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_f01"));
    }
    frame(fighter.lua_state_agent, 87.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appeal01_02"));
    }
}

//AppealSR
unsafe extern "C" fn sound_maskedman_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_landing01"));
    }
}

//AppealSL
unsafe extern "C" fn sound_maskedman_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_landing01"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_maskedman_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_l02"));
    }
    wait(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_maskedman_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_special_l02"));
    }
    wait(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
    }
}

//Final
unsafe extern "C" fn sound_maskedman_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 33.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_special_h02"));
        }
        wait(fighter.lua_state_agent, 8.0);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_special_n04_ll"));
    }
}

pub fn install() {
    Agent::new("lucas")
    .sound_acmd("sound_attack11_maskedman", sound_maskedman_Attack11, Low)
    .sound_acmd("sound_attackdash_maskedman", sound_maskedman_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_maskedman", sound_maskedman_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_maskedman", sound_maskedman_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_maskedman", sound_maskedman_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_maskedman", sound_maskedman_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_maskedman", sound_maskedman_AttackLw3, Low)
    .sound_acmd("sound_attacks4charge_maskedman", sound_maskedman_AttackS4Charge, Low)
    .sound_acmd("sound_attacks4_maskedman", sound_maskedman_AttackS4, Low)
    .sound_acmd("sound_attackhi4charge_maskedman", sound_maskedman_AttackHi4Charge, Low)
    .sound_acmd("sound_attackhi4_maskedman", sound_maskedman_AttackHi4, Low)
    .sound_acmd("sound_attacklw4charge_maskedman", sound_maskedman_AttackLw4Charge, Low)
    .sound_acmd("sound_attacklw4_maskedman", sound_maskedman_AttackLw4, Low)
    .sound_acmd("sound_attackairn_maskedman", sound_maskedman_AttackAirN, Low)
    .sound_acmd("sound_attackairf_maskedman", sound_maskedman_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_maskedman", sound_maskedman_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_maskedman", sound_maskedman_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_maskedman", sound_maskedman_AttackAirLw, Low)
    .sound_acmd("sound_catch_maskedman", sound_maskedman_Catch, Low)
    .sound_acmd("sound_catchdash_maskedman", sound_maskedman_CatchDash, Low)
    .sound_acmd("sound_catchturn_maskedman", sound_maskedman_CatchTurn, Low)
    .sound_acmd("sound_throwf_maskedman", sound_maskedman_ThrowF, Low)
    .sound_acmd("sound_throwb_maskedman", sound_maskedman_ThrowB, Low)
    .sound_acmd("sound_throwhi_maskedman", sound_maskedman_ThrowHi, Low)
    .sound_acmd("sound_throwlw_maskedman", sound_maskedman_ThrowLw, Low)
    .sound_acmd("sound_downattackd_maskedman", sound_maskedman_DownAttackD, Low)
    .sound_acmd("sound_downattacku_maskedman", sound_maskedman_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_maskedman", sound_maskedman_CliffAttack, Low)
    .sound_acmd("sound_slipattack_maskedman", sound_maskedman_SlipAttack, Low)
    .sound_acmd("sound_specialnstart_maskedman", sound_maskedman_SpecialNStart, Low)
    .sound_acmd("sound_specialairnstart_maskedman", sound_maskedman_SpecialAirNStart, Low)
    .sound_acmd("sound_specialndash_maskedman", sound_maskedman_SpecialNDash, Low)
    .sound_acmd("sound_specialairndash_maskedman", sound_maskedman_SpecialAirNDash, Low)
    .sound_acmd("sound_specials_maskedman", sound_maskedman_SpecialS, Low)
    .sound_acmd("sound_specialairs_maskedman", sound_maskedman_SpecialAirS, Low)
    .sound_acmd("sound_specialhistart_maskedman", sound_maskedman_SpecialHiStart, Low)
    .sound_acmd("sound_specialhihold_maskedman", sound_maskedman_SpecialHiHold, Low)
    .sound_acmd("sound_specialairhistart_maskedman", sound_maskedman_SpecialAirHiStart, Low)
    .sound_acmd("sound_speciallwstart_maskedman", sound_maskedman_SpecialLwStart, Low)
    .sound_acmd("sound_specialairlwstart_maskedman", sound_maskedman_SpecialAirLwStart, Low)
    .sound_acmd("sound_appealhir_maskedman", sound_maskedman_AppealHiR, Low)
    .sound_acmd("sound_appealhil_maskedman", sound_maskedman_AppealHiL, Low)
    .sound_acmd("sound_appealsr_maskedman", sound_maskedman_AppealSR, Low)
    .sound_acmd("sound_appealsl_maskedman", sound_maskedman_AppealSL, Low)
    .sound_acmd("sound_appeallwr_maskedman", sound_maskedman_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_maskedman", sound_maskedman_AppealLwL, Low)
    .sound_acmd("sound_final_maskedman", sound_maskedman_Final, Low)
    .install();
}