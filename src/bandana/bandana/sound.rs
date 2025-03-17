use crate::imports::BuildImports::*;

//WalkSlow
unsafe extern "C" fn sound_bandana_WalkSlow(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_left_s"));
        }
        frame(fighter.lua_state_agent, 42.0);
        PLAY_STEP(fighter, Hash40::new("se_edge_step_right_s"));
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//WalkMiddle
unsafe extern "C" fn sound_bandana_WalkMiddle(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_left_m"));
        }
        frame(fighter.lua_state_agent, 30.0);
        PLAY_STEP(fighter, Hash40::new("se_edge_step_right_m"));
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//WalkFast
unsafe extern "C" fn sound_bandana_WalkFast(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_left_m"));
        }
        frame(fighter.lua_state_agent, 20.0);
        PLAY_STEP(fighter, Hash40::new("se_edge_step_right_m"));
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//Run
unsafe extern "C" fn sound_bandana_Run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    loop {
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_left_m"));
        }
        wait(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_right_m"));
        }
        wait(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_left_m"));
        }
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_STEP(fighter, Hash40::new("se_edge_step_right_m"));
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//Attack11
unsafe extern "C" fn sound_bandana_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
}

//Attack12
unsafe extern "C" fn sound_bandana_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
}

//Attack13
unsafe extern "C" fn sound_bandana_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
}

//Attack100 
unsafe extern "C" fn sound_bandana_Attack100(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_attack09"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackair_f03_1"));
    }
}

//Attack100End
unsafe extern "C" fn sound_bandana_Attack100End(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_attackair_f03_1"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//AttackDash
unsafe extern "C" fn sound_bandana_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackdash01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_turn"));
    }
    frame(fighter.lua_state_agent, 51.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_right_s"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_bandana_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
    }
}

//AttackS3
unsafe extern "C" fn sound_bandana_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_bandana_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_bandana_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_01"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_bandana_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_edge_final02_03"));
    }
}

//AttackS4
unsafe extern "C" fn sound_bandana_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_m01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackhard_s02"));
        PLAY_SE(fighter, Hash40::new("vc_edge_attack08"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_bandana_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
        PLAY_SE(fighter, Hash40::new("se_edge_smash_h02"));
        PLAY_SE(fighter, Hash40::new("vc_edge_attack05"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_bandana_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_attack07"));
        PLAY_SE(fighter, Hash40::new("se_edge_attackair_f04"));
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackair_f04"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_bandana_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_bandana_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_m01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_bandana_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
}        

//AttackAirHi
unsafe extern "C" fn sound_bandana_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackair_h02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_bandana_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackair_l01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
}

//LandingAirLw
unsafe extern "C" fn sound_bandana_LandingAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_m_01"));
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n02"));
    }
}

//Catch
unsafe extern "C" fn sound_bandana_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
}

//CatchDash
unsafe extern "C" fn sound_bandana_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
    wait(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_m"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_m"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_landing01"));
    }
}

//CatchTurn
unsafe extern "C" fn sound_bandana_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
    }
}

//ThrowF
unsafe extern "C" fn sound_bandana_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    frame(fighter.lua_state_agent, 6.0);
    for _ in 0..7 {
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_magic_hit_s"));
        }
        wait(fighter.lua_state_agent, 4.0);
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_magic_hit_m"));
    }
}  

//ThrowB
unsafe extern "C" fn sound_bandana_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}  

//ThrowHi
unsafe extern "C" fn sound_bandana_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}   

//ThrowLw
unsafe extern "C" fn sound_bandana_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_bandana_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_start"));
    }
    wait(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_landing01"));
    }
}

//SlipAttack
unsafe extern "C" fn sound_bandana_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_landing01"));
    }
}

//DownAttackD
unsafe extern "C" fn sound_bandana_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_m01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_bandana_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_swing_l01"));
    }
}

//SpecialNStart
unsafe extern "C" fn sound_bandana_SpecialNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
        PLAY_SE(fighter, Hash40::new("se_edge_special_n03_01"));
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialAirNStart
unsafe extern "C" fn sound_bandana_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
        PLAY_SE(fighter, Hash40::new("se_edge_special_n03_01"));
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialN1
unsafe extern "C" fn sound_bandana_SpecialN1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialAirN1
unsafe extern "C" fn sound_bandana_SpecialAirN1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialN2
unsafe extern "C" fn sound_bandana_SpecialN2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialAirN2
unsafe extern "C" fn sound_bandana_SpecialAirN2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_n02_03"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_n01"));
    }
}

//SpecialS
unsafe extern "C" fn sound_bandana_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_s01"));
        if !CustomModule::is_exist_fire(fighter.module_accessor) {
            PLAY_SE(fighter, Hash40::new("se_edge_special_n04_01"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_common_hit_nodamage"));
        }
    }
}

//SpecialAirS
unsafe extern "C" fn sound_bandana_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_special_s01"));
        if !CustomModule::is_exist_fire(fighter.module_accessor) {
            PLAY_SE(fighter, Hash40::new("se_edge_special_n04_01"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("se_common_hit_nodamage"));
        }
    }
}

//SpecialSHold
unsafe extern "C" fn sound_bandana_SpecialSHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
        PLAY_SE(fighter, Hash40::new("se_edge_special_n04_03"));
    }
    loop {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
        }
        wait(fighter.lua_state_agent, 6.0);
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//SpecialAirSHold
unsafe extern "C" fn sound_bandana_SpecialAirSHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
        PLAY_SE(fighter, Hash40::new("se_edge_special_n04_03"));
    }
    loop {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_common_swing_02"));
        }
        wait(fighter.lua_state_agent, 6.0);
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

//SpecialHi
unsafe extern "C" fn sound_bandana_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
    frame(fighter.lua_state_agent, 96.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h02"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_bandana_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
    frame(fighter.lua_state_agent, 96.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h02"));
    }
}

//SpecialLw
unsafe extern "C" fn sound_bandana_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_l02_02"));
    }
}

//SpecialAirLw
unsafe extern "C" fn sound_bandana_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_edge_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_l02_02"));
    }
}

//SpecialAirLwLand
unsafe extern "C" fn sound_bandana_SpecialAirLwLand(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_l04"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_bandana_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_rise"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_bandana_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_rise"));
    }
}

//AppealSR
unsafe extern "C" fn sound_bandana_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_l02_01"));
    }
}

//AppealSL
unsafe extern "C" fn sound_bandana_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 72.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_l02_01"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_bandana_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_s"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_s"));
    }
    frame(fighter.lua_state_agent, 74.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_system_infowindow_open"));
        PLAY_SE(fighter, Hash40::new("se_edge_appeal_l01"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_bandana_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_s"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_step_left_s"));
    }
    frame(fighter.lua_state_agent, 74.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_system_infowindow_open"));
        PLAY_SE(fighter, Hash40::new("se_edge_appeal_l01"));
    }
}

//FinalStart
unsafe extern "C" fn sound_bandana_FinalStart(fighter: &mut L2CAgentBase) { 
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_jump04"));
        PLAY_SE(fighter, Hash40::new("se_edge_final01_01"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_final01_02"));
    }
} 

//FinalHold
unsafe extern "C" fn sound_bandana_FinalHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_final01"));
    }
} 

//FinalEnd
unsafe extern "C" fn sound_bandana_FinalEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_final01_02"));
        PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_l"));
    }
} 

//FinalAirStart
unsafe extern "C" fn sound_bandana_FinalAirStart(fighter: &mut L2CAgentBase) { 
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_jump04"));
        PLAY_SE(fighter, Hash40::new("se_edge_final01_01"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_final01_02"));
    }
} 

//FinalAirHold
unsafe extern "C" fn sound_bandana_FinalAirHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_edge_final01"));
    }
} 

//FinalAirEnd
unsafe extern "C" fn sound_bandana_FinalAirEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_final01_02"));
        PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_l"));
    }
} 

//EntryL
unsafe extern "C" fn sound_bandana_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
}

//EntryR
unsafe extern "C" fn sound_bandana_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
    frame(fighter.lua_state_agent, 79.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_edge_special_h01"));
    }
}

//Win1
unsafe extern "C" fn sound_bandana_Win1(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
        PLAY_SE(fighter, Hash40::new("se_edge_appear01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_start"));
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_start"));
    }
    frame(fighter.lua_state_agent, 109.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_start"));
    }
}

//Win2
unsafe extern "C" fn sound_bandana_Win2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
        PLAY_SE(fighter, Hash40::new("se_edge_appear01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
    }
    frame(fighter.lua_state_agent, 137.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_start"));
    }
    frame(fighter.lua_state_agent, 163.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_edge_dash_stop"));
    }
}

//Win3
unsafe extern "C" fn sound_bandana_Win3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
        PLAY_SE(fighter, Hash40::new("se_edge_appear01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        the_csk_collection_api::play_bgm(0x18859f471e);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_bandana_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_edge_rnd_futtobi01"), Hash40::new("seq_edge_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_bandana_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_edge_rnd_futtobi01"), Hash40::new("seq_edge_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_bandana_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_edge_rnd_futtobi01"), Hash40::new("seq_edge_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_bandana_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_edge_rnd_futtobi01"), Hash40::new("seq_edge_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_bandana_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_edge_rnd_futtobi01"), Hash40::new("seq_edge_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("edge")
    .sound_acmd("sound_walkslow_bandana", sound_bandana_WalkSlow, Low)
    .sound_acmd("sound_walkmiddle_bandana", sound_bandana_WalkMiddle, Low)
    .sound_acmd("sound_walkfast_bandana", sound_bandana_WalkFast, Low)
    .sound_acmd("sound_run_bandana", sound_bandana_Run, Low)
    .sound_acmd("sound_attack11_bandana", sound_bandana_Attack11, Low)
    .sound_acmd("sound_attack12_bandana", sound_bandana_Attack12, Low)
    .sound_acmd("sound_attack13_bandana", sound_bandana_Attack13, Low)
    .sound_acmd("sound_attack100_bandana", sound_bandana_Attack100, Low)
    .sound_acmd("sound_attack100end_bandana", sound_bandana_Attack100End, Low)
    .sound_acmd("sound_attackdash_bandana", sound_bandana_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_bandana", sound_bandana_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_bandana", sound_bandana_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_bandana", sound_bandana_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_bandana", sound_bandana_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_bandana", sound_bandana_AttackLw3, Low)
    .sound_acmd("sound_attacks4_bandana", sound_bandana_AttackS4, Low)
    .sound_acmd("sound_attackhi4_bandana", sound_bandana_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_bandana", sound_bandana_AttackLw4, Low)
    .sound_acmd("sound_attackairn_bandana", sound_bandana_AttackAirN, Low)
    .sound_acmd("sound_attackairf_bandana", sound_bandana_AttackAirF, Low) 
    .sound_acmd("sound_attackairb_bandana", sound_bandana_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_bandana", sound_bandana_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_bandana", sound_bandana_AttackAirLw, Low)
    .sound_acmd("sound_landingairlw_bandana", sound_bandana_LandingAirLw, Low)
    .sound_acmd("sound_catch_bandana", sound_bandana_Catch, Low)
    .sound_acmd("sound_catchdash_bandana", sound_bandana_CatchDash, Low)
    .sound_acmd("sound_catchturn_bandana", sound_bandana_CatchTurn, Low)
    .sound_acmd("sound_throwf_bandana", sound_bandana_ThrowF, Low)
    .sound_acmd("sound_throwb_bandana", sound_bandana_ThrowB, Low)
    .sound_acmd("sound_throwhi_bandana", sound_bandana_ThrowHi, Low)
    .sound_acmd("sound_throwlw_bandana", sound_bandana_ThrowLw, Low)
    .sound_acmd("sound_downattackd_bandana", sound_bandana_DownAttackD, Low)
    .sound_acmd("sound_downattacku_bandana", sound_bandana_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_bandana", sound_bandana_CliffAttack, Low)
    .sound_acmd("sound_slipattack_bandana", sound_bandana_SlipAttack, Low)
    .sound_acmd("sound_specialnstart_bandana", sound_bandana_SpecialNStart, Low)
    .sound_acmd("sound_specialairnstart_bandana", sound_bandana_SpecialAirNStart, Low)
    .sound_acmd("sound_specialn1_bandana", sound_bandana_SpecialN1, Low)
    .sound_acmd("sound_specialairn1_bandana", sound_bandana_SpecialAirN1, Low)
    .sound_acmd("sound_specialn2_bandana", sound_bandana_SpecialN2, Low)
    .sound_acmd("sound_specialairn2_bandana", sound_bandana_SpecialAirN2, Low)
    .sound_acmd("sound_specials_bandana", sound_bandana_SpecialS, Low)
    .sound_acmd("sound_specialairs_bandana", sound_bandana_SpecialAirS, Low)
    .sound_acmd("sound_specialshold_bandana", sound_bandana_SpecialSHold, Low)
    .sound_acmd("sound_specialairshold_bandana", sound_bandana_SpecialAirSHold, Low)
    .sound_acmd("sound_specialhi_bandana", sound_bandana_SpecialHi, Low)
    .sound_acmd("sound_specialairhi_bandana", sound_bandana_SpecialAirHi, Low)
    .sound_acmd("sound_speciallw_bandana", sound_bandana_SpecialLw, Low)
    .sound_acmd("sound_specialairlw_bandana", sound_bandana_SpecialAirLw, Low)
    .sound_acmd("sound_specialairlwland_bandana", sound_bandana_SpecialAirLwLand, Low)
    .sound_acmd("sound_appealsr_bandana", sound_bandana_AppealSR, Low)
    .sound_acmd("sound_appealsl_bandana", sound_bandana_AppealSL, Low)
    .sound_acmd("sound_appealhir_bandana", sound_bandana_AppealHiR, Low)
    .sound_acmd("sound_appealhil_bandana", sound_bandana_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_bandana", sound_bandana_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_bandana", sound_bandana_AppealLwL, Low)
    .sound_acmd("sound_final_bandana", sound_bandana_FinalStart, Low)
    .sound_acmd("sound_finalhold_bandana", sound_bandana_FinalHold, Low)
    .sound_acmd("sound_finalend_bandana", sound_bandana_FinalEnd, Low)
    .sound_acmd("sound_finalair_bandana", sound_bandana_FinalAirStart, Low)
    .sound_acmd("sound_finalairhold_bandana", sound_bandana_FinalAirHold, Low)
    .sound_acmd("sound_finalairend_bandana", sound_bandana_FinalAirEnd, Low)
    .sound_acmd("sound_entryl_bandana", sound_bandana_EntryL, Low)
    .sound_acmd("sound_entryr_bandana", sound_bandana_EntryR, Low)
    .sound_acmd("sound_win1_bandana", sound_bandana_Win1, Low)
    .sound_acmd("sound_win2_bandana", sound_bandana_Win2, Low)
    .sound_acmd("sound_win3_bandana", sound_bandana_Win3, Low)
    .sound_acmd("sound_damageflyhi_bandana", sound_bandana_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw_bandana", sound_bandana_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn_bandana", sound_bandana_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop_bandana", sound_bandana_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll_bandana", sound_bandana_DamageFlyRoll, Low)
    .install();
}