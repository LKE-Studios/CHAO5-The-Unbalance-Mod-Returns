use crate::imports::BuildImports::*;

//JumpFront
unsafe extern "C" fn sound_kamek_JumpFront(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_jump01"));
    }
}

//JumpBack
unsafe extern "C" fn sound_kamek_JumpBack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_jump01"));
    }
}

//JumpAerial
unsafe extern "C" fn sound_kamek_JumpAerial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_pk_s"));
    }
}

//JumpAerialFront
unsafe extern "C" fn sound_kamek_JumpAerialFront(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_pk_s"));
    }
}

//JumpAerialBack
unsafe extern "C" fn sound_kamek_JumpAerialBack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_pk_s"));
    }
}

//Run
unsafe extern "C" fn sound_kamek_Run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_ness_yoyo_swing"));
    }
}

//TurnRun
unsafe extern "C" fn sound_kamek_TurnRun(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}

//RunBrake
unsafe extern "C" fn sound_kamek_RunBrake(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}

//Attack11 
unsafe extern "C" fn sound_kamek_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
}

//AttackDash
unsafe extern "C" fn sound_kamek_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
}

//AttackS3
unsafe extern "C" fn sound_kamek_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_magic_hit_s"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_kamek_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_b01"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_kamek_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_kamek_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal02_02"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_kamek_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_h01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_damage02"));
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_hit_nodamage"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_kamek_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_ness_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal03"));
    }
}

//AttackAirN
unsafe extern "C" fn sound_kamek_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
}

//AttackAirF 
unsafe extern "C" fn sound_kamek_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_starrod_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirB
unsafe extern "C" fn sound_kamek_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    } 
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_b01"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_kamek_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    }
}

//AttackAirLw
unsafe extern "C" fn sound_kamek_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_ATTACK_VC(fighter);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_h01"));
    }
}

//Catch
unsafe extern "C" fn sound_kamek_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_f01"));
    }
}

//CatchDash
unsafe extern "C" fn sound_kamek_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_f01"));
    }
}

//CatchTurn
unsafe extern "C" fn sound_kamek_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_f01"));
    }
}

//ThrowF
unsafe extern "C" fn sound_kamek_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal01"));
    }
}

//ThrowB
unsafe extern "C" fn sound_kamek_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_appeal02_02"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_kamek_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk02"));
        PLAY_SE(fighter, Hash40::new("se_ness_throw_pk01"));
    }
}      

//ThrowLw
unsafe extern "C" fn sound_kamek_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_h01"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack04"));
    }
}

//CliffAttack
unsafe extern "C" fn sound_kamek_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_dash_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_s01"));
    }
}

//SpecialNHold
unsafe extern "C" fn sound_kamek_SpecialNHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
}

//SpecialAirNHold
unsafe extern "C" fn sound_kamek_SpecialAirNHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n05"));
    }
}

//SpecialNFire
unsafe extern "C" fn sound_kamek_SpecialNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack06"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n02"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_pasaran_growth"));
    }
}

//SpecialAirNFire
unsafe extern "C" fn sound_kamek_SpecialAirNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack06"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n02"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_pasaran_growth"));
    }
}

//SpecialS
unsafe extern "C" fn sound_kamek_SpecialS(fighter: &mut L2CAgentBase) {
    let magic_type = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
    let rand_num = sv_math::rand(hash40("ness"), 100);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n01"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if magic_type == 1 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
    }
}

//SpecialAirS
unsafe extern "C" fn sound_kamek_SpecialAirS(fighter: &mut L2CAgentBase) {
    let rand_num = sv_math::rand(hash40("ness"), 100);
    let magic_type = WorkModule::get_int(fighter.module_accessor, *FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_special_n01"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if magic_type == 1 {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
    }
}

//SpecialHiStart
unsafe extern "C" fn sound_kamek_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_item_bomber_ottotto"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_h01"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_kamek_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_item_bomber_ottotto"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_h01"));
    }
}

//SpecialHiEnd
unsafe extern "C" fn sound_kamek_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_n01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_h02"));
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn sound_kamek_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackair_n01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ness_special_h02"));
    }
}

//SpecialLwStart
unsafe extern "C" fn sound_kamek_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_guardon"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_kamek_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_guardon"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack01"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_kamek_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_kamek_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal03"));
    }
}

//AppealSR
unsafe extern "C" fn sound_kamek_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 92.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack07"));
    }
}

//AppealSL
unsafe extern "C" fn sound_kamek_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 92.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_attack07"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_kamek_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_kamek_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal02"));
    }
}

//Final
unsafe extern "C" fn sound_kamek_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
        PLAY_SE(fighter, Hash40::new("se_system_sprits_level_up_right_center"));
        PLAY_SE(fighter, Hash40::new("se_ness_special_n02"));
    }
}

//FinalAir
unsafe extern "C" fn sound_kamek_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ness_attackhard_h01"));
        PLAY_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_ness_appeal01"));
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_spirits_floor_ice_loop"));
        PLAY_SE(fighter, Hash40::new("vc_ness_attack05"));
        PLAY_SE(fighter, Hash40::new("se_system_sprits_level_up_right_center"));
        PLAY_SE(fighter, Hash40::new("se_ness_special_n02"));
    }
}

pub fn install() {
    Agent::new("ness")
    .sound_acmd("sound_jumpfront_kamek", sound_kamek_JumpFront, Low)
    .sound_acmd("sound_jumpback_kamek", sound_kamek_JumpBack, Low)
    .sound_acmd("sound_jumpaerial_kamek", sound_kamek_JumpAerial, Low)
    .sound_acmd("sound_jumpaerialfront_kamek", sound_kamek_JumpAerialFront, Low)
    .sound_acmd("sound_jumpaerialback_kamek", sound_kamek_JumpAerialBack, Low)
    .sound_acmd("sound_run_kamek", sound_kamek_Run, Low)
    .sound_acmd("sound_turnrun_kamek", sound_kamek_TurnRun, Low)
    .sound_acmd("sound_runbrake_kamek", sound_kamek_RunBrake, Low)
    .sound_acmd("sound_attack11_kamek", sound_kamek_Attack11, Low)
    .sound_acmd("sound_attackdash_kamek", sound_kamek_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_kamek", sound_kamek_AttackS3, Low)
    .sound_acmd("sound_attacks3_kamek", sound_kamek_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_kamek", sound_kamek_AttackS3, Low)
    .sound_acmd("sound_attackhi3_kamek", sound_kamek_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_kamek", sound_kamek_AttackLw3, Low)
    .sound_acmd("sound_attacks4_kamek", sound_kamek_AttackS4, Low)
    .sound_acmd("sound_attackhi4_kamek", sound_kamek_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_kamek", sound_kamek_AttackLw4, Low)
    .sound_acmd("sound_attackairn_kamek", sound_kamek_AttackAirN, Low)
    .sound_acmd("sound_attackairf_kamek", sound_kamek_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_kamek", sound_kamek_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_kamek", sound_kamek_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_kamek", sound_kamek_AttackAirLw, Low)
    .sound_acmd("sound_catch_kamek", sound_kamek_Catch, Low)
    .sound_acmd("sound_catchdash_kamek", sound_kamek_CatchDash, Low)
    .sound_acmd("sound_catchturn_kamek", sound_kamek_CatchTurn, Low)
    .sound_acmd("sound_throwf_kamek", sound_kamek_ThrowF, Low)
    .sound_acmd("sound_throwb_kamek", sound_kamek_ThrowB, Low)
    .sound_acmd("sound_throwhi_kamek", sound_kamek_ThrowHi, Low)
    .sound_acmd("sound_throwlw_kamek", sound_kamek_ThrowLw, Low)
    .sound_acmd("sound_cliffattack_kamek", sound_kamek_CliffAttack, Low)
    .sound_acmd("sound_specialnhold_kamek", sound_kamek_SpecialNHold, Low)  
    .sound_acmd("sound_specialairnhold_kamek", sound_kamek_SpecialAirNHold, Low)
    .sound_acmd("sound_specialnfire_kamek", sound_kamek_SpecialNFire, Low)  
    .sound_acmd("sound_specialairnfire_kamek", sound_kamek_SpecialAirNFire, Low)  
    .sound_acmd("sound_specials_kamek", sound_kamek_SpecialS, Low)  
    .sound_acmd("sound_specialairs_kamek", sound_kamek_SpecialAirS, Low) 
    .sound_acmd("sound_specialhistart_kamek", sound_kamek_SpecialHiStart, Low)
    .sound_acmd("sound_specialairhistart_kamek", sound_kamek_SpecialAirHiStart, Low)
    .sound_acmd("sound_specialhiend_kamek", sound_kamek_SpecialHiEnd, Low)
    .sound_acmd("sound_specialairhiend_kamek", sound_kamek_SpecialAirHiEnd, Low)
    .sound_acmd("sound_speciallwstart_kamek", sound_kamek_SpecialLwStart, Low)
    .sound_acmd("sound_specialairlwstart_kamek", sound_kamek_SpecialAirLwStart, Low)
    .sound_acmd("sound_appealsr_kamek", sound_kamek_AppealSR, Low)
    .sound_acmd("sound_appealsl_kamek", sound_kamek_AppealSL, Low)
    .sound_acmd("sound_appealhir_kamek", sound_kamek_AppealHiR, Low)
    .sound_acmd("sound_appealhil_kamek", sound_kamek_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_kamek", sound_kamek_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_kamek", sound_kamek_AppealLwL, Low)
    .sound_acmd("sound_final_kamek", sound_kamek_Final, Low)
    .sound_acmd("sound_finalair_kamek", sound_kamek_FinalAir, Low)
    .install();
}
