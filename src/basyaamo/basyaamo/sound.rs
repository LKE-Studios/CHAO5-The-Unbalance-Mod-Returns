use crate::imports::BuildImports::*;

//Attack11 
unsafe extern "C" fn sound_basyaamo_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
}

//Attack12 
unsafe extern "C" fn sound_basyaamo_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//Attack13 
unsafe extern "C" fn sound_basyaamo_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//AttackDash
unsafe extern "C" fn sound_basyaamo_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_ll"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_basyaamo_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackS3
unsafe extern "C" fn sound_basyaamo_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_basyaamo_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_basyaamo_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_basyaamo_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackS4
unsafe extern "C" fn sound_basyaamo_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_ll"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_basyaamo_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack06"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_basyaamo_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack07"));
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_basyaamo_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//AttackAirF 
unsafe extern "C" fn sound_basyaamo_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_basyaamo_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_basyaamo_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_basyaamo_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_captain_swing_ll"));
    }
}

//Catch
unsafe extern "C" fn sound_basyaamo_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_jump02"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_captain_jump02"));
    }
}

//CatchDash
unsafe extern "C" fn sound_basyaamo_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_captain_step_left_m"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_jump02"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_captain_jump02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_captain_step_left_m"));
    }
}

//CatchTurn
unsafe extern "C" fn sound_basyaamo_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_STEP(fighter, Hash40::new("se_captain_step_left_m"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_jump02"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_captain_jump02"));
    }
}

//ThrowF
unsafe extern "C" fn sound_basyaamo_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
    }
}

//ThrowB
unsafe extern "C" fn sound_basyaamo_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_basyaamo_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
    }
}      

//ThrowLw
unsafe extern "C" fn sound_basyaamo_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_m"));
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_captain_rnd_attack"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_basyaamo_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_cliffjump"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_ll"));
    }
}

//CliffClimb
unsafe extern "C" fn sound_basyaamo_CliffClimb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_cliffjump"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing02"));
    }
}

//CliffEscape
unsafe extern "C" fn sound_basyaamo_CliffEscape(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//SlipAttack
unsafe extern "C" fn sound_basyaamo_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
    wait(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
}

//DownAttackD
unsafe extern "C" fn sound_basyaamo_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_l"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_jump02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
}

//DownForwardD
unsafe extern "C" fn sound_basyaamo_DownForwardD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_dash_start"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_escape"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
}

//DownBackD
unsafe extern "C" fn sound_basyaamo_DownBackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_dash_start"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_escape"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_basyaamo_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_jump02"));
    }
}

//DownForwardU
unsafe extern "C" fn sound_basyaamo_DownForwardU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_dash_start"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_escape"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
}

//DownBackU
unsafe extern "C" fn sound_basyaamo_DownBackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_dash_start"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_escape"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_landing01"));
    }
}

//SpecialN
unsafe extern "C" fn sound_basyaamo_SpecialN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s01"));
    }
}

//SpecialNOverheat
unsafe extern "C" fn sound_basyaamo_SpecialNOverheat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_h01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_002"));
    }
}

//SpecialAirN
unsafe extern "C" fn sound_basyaamo_SpecialAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s01"));
    }
}

//SpecialS
unsafe extern "C" fn sound_basyaamo_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack05"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s02"));
    }
}

//SpecialAirS
unsafe extern "C" fn sound_basyaamo_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack05"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_s02"));
    }
}

//SpecialHi
unsafe extern "C" fn sound_basyaamo_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_h01"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_basyaamo_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_h01"));
    }
}

//SpecialHiOverheat
unsafe extern "C" fn sound_basyaamo_SpecialHiOverheat(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_h01"));
    }

    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack07"));
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_n04"));
    }
}

//SpecialLw
unsafe extern "C" fn sound_basyaamo_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_m"));
    }
}

//SpecialLwLoop
unsafe extern "C" fn sound_basyaamo_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_attack07"));
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_h01"));
    }
}

//SpecialLwLanding
unsafe extern "C" fn sound_basyaamo_SpecialLwLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_captain_special_h01"));
        PLAY_SE(fighter, Hash40::new("se_captain_special_h03"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_basyaamo_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_appeal01"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_basyaamo_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_appeal01"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_swing_s"));
    }
}

//AppealSR
unsafe extern "C" fn sound_basyaamo_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_hit_s"));
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_appeal02"));
    }
}

//AppealSL
unsafe extern "C" fn sound_basyaamo_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_sword_hit_s"));
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    frame(fighter.lua_state_agent, 68.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_appeal02"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_basyaamo_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_n04"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_basyaamo_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_captain_special_n04"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_captain_003"));
    }
}

//EntryR
unsafe extern "C" fn sound_basyaamo_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//EntryL
unsafe extern "C" fn sound_basyaamo_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_CAPTAIN_GENERATE_ARTICLE_BLUEFALCON, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//PassiveStandF
unsafe extern "C" fn sound_basyaamo_PassiveStandF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

//PassiveStandB
unsafe extern "C" fn sound_basyaamo_PassiveStandB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("captain")
    .sound_acmd("sound_attack11_basyaamo", sound_basyaamo_Attack11, Low)
    .sound_acmd("sound_attack12_basyaamo", sound_basyaamo_Attack12, Low)
    .sound_acmd("sound_attack13_basyaamo", sound_basyaamo_Attack13, Low)
    .sound_acmd("sound_attackdash_basyaamo", sound_basyaamo_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_basyaamo", sound_basyaamo_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_basyaamo", sound_basyaamo_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_basyaamo", sound_basyaamo_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_basyaamo", sound_basyaamo_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_basyaamo", sound_basyaamo_AttackLw3, Low)
    .sound_acmd("sound_attacks4_basyaamo", sound_basyaamo_AttackS4, Low)
    .sound_acmd("sound_attackhi4_basyaamo", sound_basyaamo_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_basyaamo", sound_basyaamo_AttackLw4, Low)
    .sound_acmd("sound_attackairn_basyaamo", sound_basyaamo_AttackAirN, Low)
    .sound_acmd("sound_attackairf_basyaamo", sound_basyaamo_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_basyaamo", sound_basyaamo_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_basyaamo", sound_basyaamo_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_basyaamo", sound_basyaamo_AttackAirLw, Low)
    .sound_acmd("sound_catch_basyaamo", sound_basyaamo_Catch, Low)
    .sound_acmd("sound_catchdash_basyaamo", sound_basyaamo_CatchDash, Low)
    .sound_acmd("sound_catchturn_basyaamo", sound_basyaamo_CatchTurn, Low)
    .sound_acmd("sound_throwf_basyaamo", sound_basyaamo_ThrowF, Low)
    .sound_acmd("sound_throwb_basyaamo", sound_basyaamo_ThrowB, Low)
    .sound_acmd("sound_throwhi_basyaamo", sound_basyaamo_ThrowHi, Low)
    .sound_acmd("sound_throwlw_basyaamo", sound_basyaamo_ThrowLw, Low)
    .sound_acmd("sound_downattackd_basyaamo", sound_basyaamo_DownAttackD, Low)
    .sound_acmd("sound_downforwardd_basyaamo", sound_basyaamo_DownForwardD, Low)
    .sound_acmd("sound_downbackd_basyaamo", sound_basyaamo_DownBackD, Low)
    .sound_acmd("sound_downattacku_basyaamo", sound_basyaamo_DownAttackU, Low)
    .sound_acmd("sound_downforwardu_basyaamo", sound_basyaamo_DownForwardU, Low)
    .sound_acmd("sound_downbacku_basyaamo", sound_basyaamo_DownBackU, Low)
    .sound_acmd("sound_cliffattack_basyaamo", sound_basyaamo_CliffAttack, Low)
    .sound_acmd("sound_cliffclimb_basyaamo", sound_basyaamo_CliffClimb, Low)
    .sound_acmd("sound_cliffescape_basyaamo", sound_basyaamo_CliffEscape, Low)
    .sound_acmd("sound_slipattack_basyaamo", sound_basyaamo_SlipAttack, Low)
    .sound_acmd("sound_specialn_basyaamo", sound_basyaamo_SpecialN, Low)
    .sound_acmd("sound_specialairn_basyaamo", sound_basyaamo_SpecialAirN, Low)
    .sound_acmd("sound_specialnoverheat_basyaamo", sound_basyaamo_SpecialNOverheat, Low)
    .sound_acmd("sound_specials_basyaamo", sound_basyaamo_SpecialS, Low)
    .sound_acmd("sound_specialairs_basyaamo", sound_basyaamo_SpecialAirS, Low)
    .sound_acmd("sound_specialhi_basyaamo", sound_basyaamo_SpecialHi, Low)
    .sound_acmd("sound_specialairhi_basyaamo", sound_basyaamo_SpecialAirHi, Low)
    .sound_acmd("sound_specialhioverheat_basyaamo", sound_basyaamo_SpecialHiOverheat, Low)
    .sound_acmd("sound_speciallw_basyaamo", sound_basyaamo_SpecialLw, Low)
    .sound_acmd("sound_speciallwloop_basyaamo", sound_basyaamo_SpecialLwLoop, Low)
    .sound_acmd("sound_speciallwlanding_basyaamo", sound_basyaamo_SpecialLwLanding, Low)
    .sound_acmd("sound_appealsr_basyaamo", sound_basyaamo_AppealSR, Low)
    .sound_acmd("sound_appealsl_basyaamo", sound_basyaamo_AppealSL, Low)
    .sound_acmd("sound_appealhir_basyaamo", sound_basyaamo_AppealHiR, Low)
    .sound_acmd("sound_appealhil_basyaamo", sound_basyaamo_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_basyaamo", sound_basyaamo_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_basyaamo", sound_basyaamo_AppealLwL, Low)
    .sound_acmd("sound_entryr_basyaamo", sound_basyaamo_EntryR, Low)
    .sound_acmd("sound_entryl_basyaamo", sound_basyaamo_EntryL, Low)
    .sound_acmd("sound_passivestandf_basyaamo", sound_basyaamo_PassiveStandF, Low)
    .sound_acmd("sound_passivestandb_basyaamo", sound_basyaamo_PassiveStandB, Low)
    .install();
}