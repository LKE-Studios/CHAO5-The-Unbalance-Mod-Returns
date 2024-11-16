use crate::imports::BuildImports::*;

//Attack11W
unsafe extern "C" fn effect_ken_Attack11W(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 9.5, 4, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 360, false, 0.3);
    }
}

//Attack12
unsafe extern "C" fn effect_ken_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 8.8, 15.5, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 360, false, 0.3);
    }
}

//Attack13
unsafe extern "C" fn effect_ken_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("ken_attack_arc"), Hash40::new("ken_attack_arc"), Hash40::new("top"), 0, 12.5, 2.5, -35, -65, 0, 1.0, true, *EF_FLIP_YZ, 0.35);
    }
}

//AttackS3S
unsafe extern "C" fn effect_ken_AttackS3S(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("trans"), 4, 14, -3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ, 0.6);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 14, 13.5, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}

//AttackS3S2
unsafe extern "C" fn effect_ken_AttackS3S2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 2, 14, 5, 0, 0, 0, 1, true, *EF_FLIP_YZ, 0.7);
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

//AttackS3W
unsafe extern "C" fn effect_ken_AttackS3W(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 2, 12, 9, 0, 0, 0, 1, true, *EF_FLIP_YZ, 0.7);
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("sys_atk_smoke"), Hash40::new("top"), 1, 0, -2, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_YZ);
    }
}

//AttackHi3W
unsafe extern "C" fn effect_ken_AttackHi3W(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 8, 15.5, 1.5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, false, 0.4);
    }
}

//AttackLw3W
unsafe extern "C" fn effect_ken_AttackLw3W(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 1.5, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 0.4);
    }
}

pub fn install() {
    Agent::new("ken")
    .effect_acmd("effect_attack11w", effect_ken_Attack11W, Low)
    .effect_acmd("effect_attack11s", effect_ken_Attack11S, Low)
    .effect_acmd("effect_attack11nears", effect_ken_Attack11NearS, Low)
    .effect_acmd("effect_attack12", effect_ken_Attack12, Low)
    .effect_acmd("effect_attack13", effect_ken_Attack13, Low)
    .effect_acmd("effect_attackdash", effect_ken_AttackDash, Low)
    .effect_acmd("effect_attacks3s", effect_ken_AttackS3S, Low)
    .effect_acmd("effect_attacks3w", effect_ken_AttackS3W, Low)
    .effect_acmd("effect_attacknearw", effect_ken_AttackNearW, Low)
    .effect_acmd("effect_attackcommand1", effect_ken_AttackCommand1, Low)
    .effect_acmd("effect_attackcommand2", effect_ken_AttackCommand2, Low)
    .effect_acmd("effect_attackcommand3", effect_ken_AttackCommand3, Low)
    .effect_acmd("effect_attackhi3w", effect_ken_AttackHi3W, Low)
    .effect_acmd("effect_attackhi3s", effect_ken_AttackHi3S, Low)
    .effect_acmd("effect_attacklw3w", effect_ken_AttackLw3W, Low)
    .effect_acmd("effect_attacklw3s", effect_ken_AttackLw3S, Low)
    .effect_acmd("effect_attacks4", effect_ken_AttackS4, Low)
    .effect_acmd("effect_attackhi4", effect_ken_AttackHi4, Low)
    .effect_acmd("effect_attacklw4", effect_ken_AttackLw4, Low)
    .effect_acmd("effect_attackairn", effect_ken_AttackAirN, Low)
    .effect_acmd("effect_attackairf", effect_ken_AttackAirF, Low)    
    .effect_acmd("effect_attackairb", effect_ken_AttackAirB, Low)
    .effect_acmd("effect_attackairhi", effect_ken_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw", effect_ken_AttackAirLw, Low)
    .effect_acmd("effect_catch", effect_ken_Catch, Low)
    .effect_acmd("effect_catchdash", effect_ken_CatchDash, Low)
    .effect_acmd("effect_catchturn", effect_ken_CatchTurn, Low)
    .effect_acmd("effect_catchattack", effect_ken_CatchAttack, Low)
    .effect_acmd("effect_throwf", effect_ken_ThrowF, Low)
    .effect_acmd("effect_throwb", effect_ken_ThrowB, Low)
    .effect_acmd("effect_throwhi", effect_ken_ThrowHi, Low)
    .effect_acmd("effect_throwlw", effect_ken_ThrowLw, Low)
    .effect_acmd("effect_downattackd", effect_ken_DownAttackD, Low)
    .effect_acmd("effect_downattacku", effect_ken_DownAttackU, Low)
    .effect_acmd("effect_cliffattack", effect_ken_CliffAttack, Low)
    .effect_acmd("effect_slipattack", effect_ken_SlipAttack, Low)
    .effect_acmd("effect_specialn", effect_ken_SpecialN, Low)
    .effect_acmd("effect_specialairn", effect_ken_SpecialAirN, Low)
    .effect_acmd("effect_specialsstart", effect_ken_SpecialSStart, Low)
    .effect_acmd("effect_specials", effect_ken_SpecialS, Low)
    .effect_acmd("effect_specialairsstart", effect_ken_SpecialAirSStart, Low)
    .effect_acmd("effect_specialairs", effect_ken_SpecialAirS, Low)
    .effect_acmd("effect_specialhi", effect_ken_SpecialHi, Low)
    .effect_acmd("effect_specialairhi", effect_ken_SpecialAirHi, Low)
    .effect_acmd("effect_specialhicommand", effect_ken_SpecialHiCommand, Low)
    .effect_acmd("effect_specialairhicommand", effect_ken_SpecialAirHiCommand, Low)
    .effect_acmd("effect_speciallw", effect_ken_SpecialLw, Low)
    .effect_acmd("effect_specialairlw", effect_ken_SpecialAirLw, Low)
    .effect_acmd("effect_speciallwturn", effect_ken_SpecialLwTurn, Low)
    .effect_acmd("effect_specialairlwturn", effect_ken_SpecialAirLwTurn, Low)
    .effect_acmd("effect_appealsr", effect_ken_AppealSR, Low)
    .effect_acmd("effect_appealsl", effect_ken_AppealSL, Low)
    .effect_acmd("effect_appealhir", effect_ken_AppealHiR, Low)
    .effect_acmd("effect_appealhil", effect_ken_AppealHiL, Low)
    .effect_acmd("effect_appeallwr", effect_ken_AppealLwR, Low)
    .effect_acmd("effect_appeallwl", effect_ken_AppealLwL, Low)
    .effect_acmd("effect_finalhit", effect_ken_FinalHit, Low)
    .effect_acmd("effect_finalhit_com", effect_ken_FinalHit_Com, Low)
    .install();
}
