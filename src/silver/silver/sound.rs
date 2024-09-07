use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn sound_silver_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_h01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackDash
unsafe extern "C" fn sound_silver_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackS3Hi
unsafe extern "C" fn sound_silver_AttackS3Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackS3
unsafe extern "C" fn sound_silver_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackS3Lw
unsafe extern "C" fn sound_silver_AttackS3Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackHi3
unsafe extern "C" fn sound_silver_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackLw3 
unsafe extern "C" fn sound_silver_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_b01"));
    }
}

//AttackS4Hi
unsafe extern "C" fn sound_silver_AttackS4Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_silver_attack06"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_l02"));
    }
}

//AttackS4
unsafe extern "C" fn sound_silver_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_silver_attack06"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_l02"));
    }
}

//AttackS4Lw
unsafe extern "C" fn sound_silver_AttackS4Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_silver_attack06"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_l02"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_silver_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack04"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_swing_s"));
    }
}

//AttackLw4 
unsafe extern "C" fn sound_silver_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack05"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_n02"));
    }
}

//AttackAirN 
unsafe extern "C" fn sound_silver_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l02"));
        PLAY_SE(fighter, Hash40::new("se_silver_attackair_n02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackAirF 
unsafe extern "C" fn sound_silver_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_silver_attackair_f02"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n08"));
    }
}

//LandingAirF 
unsafe extern "C" fn sound_silver_LandingAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_mewtwo_final_01"));
        STOP_SE(fighter, Hash40::new("se_mewtwo_special_n08"));
    }
}

//AttackAirB 
unsafe extern "C" fn sound_silver_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackAirHi 
unsafe extern "C" fn sound_silver_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_f01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//AttackAirLw 
unsafe extern "C" fn sound_silver_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_f02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//ThrowF 
unsafe extern "C" fn sound_silver_ThrowF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
    }
}

//ThrowB 
unsafe extern "C" fn sound_silver_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_b"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_b02"));
    }
}

//ThrowHi 
unsafe extern "C" fn sound_silver_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_f"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_b02"));
    }
}

//ThrowLw 
unsafe extern "C" fn sound_silver_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_attackair_n01"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_n02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

//CliffAttack 
unsafe extern "C" fn sound_silver_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

//DownAttackU 
unsafe extern "C" fn sound_silver_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_rise"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_landing01"));
    }
}

//DownAttackD 
unsafe extern "C" fn sound_silver_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_rise"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l01"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_landing01"));
    }
}

//SpecialNShoot
unsafe extern "C" fn sound_silver_SpecialNShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escapeair"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s03"));
    }
}

//SpecialAirNShoot 
unsafe extern "C" fn sound_silver_SpecialAirNShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escapeair"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s03"));
    }
}

//SpecialSStart 
unsafe extern "C" fn sound_silver_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100"));
    }
}

//SpecialS 
unsafe extern "C" fn sound_silver_SpecialS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_special_n01"));
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        let rand_sound = sv_math::rand(hash40("mewtwo"), 6);
        if rand_sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s02"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s01"));
        }
    }
}

//SpecialAirSStart 
unsafe extern "C" fn sound_silver_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100"));
    }
}

//SpecialAirS 
unsafe extern "C" fn sound_silver_SpecialAirS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_special_n01"));
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        let rand_sound = sv_math::rand(hash40("mewtwo"), 6);
        if rand_sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s02"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s01"));
        }
    }
}

//SpecialHi
unsafe extern "C" fn sound_silver_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

//SpecialAirHi
unsafe extern "C" fn sound_silver_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

//SpecialLw 
unsafe extern "C" fn sound_silver_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_l01"));
    }
}

//SpecialAirLw 
unsafe extern "C" fn sound_silver_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_l01"));
    }
}

//Run 
unsafe extern "C" fn sound_silver_Run(fighter: &mut L2CAgentBase) {
    for _ in 0..36 {
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_mewtwo_step_lp_m"));
        }
        wait(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_mewtwo_step_lp_m"));
        }
    }
}

//RunBrakeR 
unsafe extern "C" fn sound_silver_RunBrakeR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

//RunBrakeL 
unsafe extern "C" fn sound_silver_RunBrakeL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

//TurnRun 
unsafe extern "C" fn sound_silver_TurnRun(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

//EscapeAir 
unsafe extern "C" fn sound_silver_EscapeAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape02"));
    }
}

//EscapeAirSlide 
unsafe extern "C" fn sound_silver_EscapeAirSlide(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape02"));
    }
}

//EscapeN 
unsafe extern "C" fn sound_silver_EscapeN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape"));
    }
}

//EscapeF 
unsafe extern "C" fn sound_silver_EscapeF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
    }
}

//EscapeB 
unsafe extern "C" fn sound_silver_EscapeB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
    }
}

//AppealHiL 
unsafe extern "C" fn sound_silver_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_silver_appeal_h02"));
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_silver_appeal_h02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_silver_appeal_h02"), 1.2);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_mewtwo_appeal01"));
    }
}

//AppealHiR 
unsafe extern "C" fn sound_silver_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_silver_appeal_h02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_silver_appeal_h02"), 1.0);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_silver_appeal_h02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_silver_appeal_h02"), 1.2);
    }
    frame(fighter.lua_state_agent, 57.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_mewtwo_appeal01"));
    }
}

//AppealSL 
unsafe extern "C" fn sound_silver_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_jump02"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_win3"));
    }
}

//AppealSR 
unsafe extern "C" fn sound_silver_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_jump02"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_win3"));
    }
}

//AppealLwL 
unsafe extern "C" fn sound_silver_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

//AppealLwR 
unsafe extern "C" fn sound_silver_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

//Win1 
unsafe extern "C" fn sound_silver_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
    }
}

//Win2 
unsafe extern "C" fn sound_silver_Win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_mewtwo_win01_02"));
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_jump02"));
    }
}

//Win3 
unsafe extern "C" fn sound_silver_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_mewtwo_win03_02"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_jump01"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_silver_appeal_h02"));
    }
    frame(fighter.lua_state_agent, 108.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .sound_acmd("sound_attack11_silver", sound_silver_Attack11, Low)
    .sound_acmd("sound_attackdash_silver", sound_silver_AttackDash, Low)
    .sound_acmd("sound_attacks3hi_silver", sound_silver_AttackS3Hi, Low)
    .sound_acmd("sound_attacks3_silver", sound_silver_AttackS3, Low)
    .sound_acmd("sound_attacks3lw_silver", sound_silver_AttackS3Lw, Low)
    .sound_acmd("sound_attackhi3_silver", sound_silver_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_silver", sound_silver_AttackLw3, Low)
    .sound_acmd("sound_attacks4hi_silver", sound_silver_AttackS4Hi, Low)
    .sound_acmd("sound_attacks4_silver", sound_silver_AttackS4, Low)
    .sound_acmd("sound_attacks4lw_silver", sound_silver_AttackS4Lw, Low)
    .sound_acmd("sound_attackhi4_silver", sound_silver_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_silver", sound_silver_AttackLw4, Low)
    .sound_acmd("sound_attackairn_silver", sound_silver_AttackAirN, Low)
    .sound_acmd("sound_attackairf_silver", sound_silver_AttackAirF, Low) 
    .sound_acmd("sound_landingairf_silver", sound_silver_LandingAirF, Low)    
    .sound_acmd("sound_attackairb_silver", sound_silver_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_silver", sound_silver_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_silver", sound_silver_AttackAirLw, Low)
    .sound_acmd("sound_throwf_silver", sound_silver_ThrowF, Low)
    .sound_acmd("sound_throwb_silver", sound_silver_ThrowB, Low)
    .sound_acmd("sound_throwhi_silver", sound_silver_ThrowHi, Low)
    .sound_acmd("sound_throwlw_silver", sound_silver_ThrowLw, Low)
    .sound_acmd("sound_downattackd_silver", sound_silver_DownAttackD, Low)
    .sound_acmd("sound_downattacku_silver", sound_silver_DownAttackU, Low)
    .sound_acmd("sound_cliffattack_silver", sound_silver_CliffAttack, Low)
    .sound_acmd("sound_specialnshoot_silver", sound_silver_SpecialNShoot, Low)
    .sound_acmd("sound_specialairnshoot_silver", sound_silver_SpecialAirNShoot, Low)
    .sound_acmd("sound_specialsstart_silver", sound_silver_SpecialSStart, Low)
    .sound_acmd("sound_specialairsstart_silver", sound_silver_SpecialAirSStart, Low)
    .sound_acmd("sound_specials_silver", sound_silver_SpecialS, Low)
    .sound_acmd("sound_specialairs_silver", sound_silver_SpecialAirS, Low)
    .sound_acmd("sound_specialhi_silver", sound_silver_SpecialHi, Low)
    .sound_acmd("sound_specialairhi_silver", sound_silver_SpecialAirHi, Low)
    .sound_acmd("sound_speciallw_silver", sound_silver_SpecialLw, Low)
    .sound_acmd("sound_specialairlw_silver", sound_silver_SpecialAirLw, Low)
    .sound_acmd("sound_run_silver", sound_silver_Run, Low)
    .sound_acmd("sound_runbrakel_silver", sound_silver_RunBrakeL, Low)
    .sound_acmd("sound_runbraker_silver", sound_silver_RunBrakeR, Low)
    .sound_acmd("sound_turnrun_silver", sound_silver_TurnRun, Low)
    .sound_acmd("sound_escapeair_silver", sound_silver_EscapeAir, Low)
    .sound_acmd("sound_escapeairslide_silver", sound_silver_EscapeAirSlide, Low)
    .sound_acmd("sound_escapen_silver", sound_silver_EscapeN, Low)
    .sound_acmd("sound_escapef_silver", sound_silver_EscapeF, Low)
    .sound_acmd("sound_escapeb_silver", sound_silver_EscapeB, Low)
    .sound_acmd("sound_win1_silver", sound_silver_Win1, Low)
    .sound_acmd("sound_win2_silver", sound_silver_Win2, Low)
    .sound_acmd("sound_win3_silver", sound_silver_Win3, Low)
    .sound_acmd("sound_appealsr_silver", sound_silver_AppealSR, Low)
    .sound_acmd("sound_appealsl_silver", sound_silver_AppealSL, Low)
    .sound_acmd("sound_appealhir_silver", sound_silver_AppealHiR, Low)
    .sound_acmd("sound_appealhil_silver", sound_silver_AppealHiL, Low)
    .sound_acmd("sound_appeallwr_silver", sound_silver_AppealLwR, Low)
    .sound_acmd("sound_appeallwl_silver", sound_silver_AppealLwL, Low)
    .install();
}