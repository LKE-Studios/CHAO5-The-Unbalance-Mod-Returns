use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "mewtwo", 
    script = "sound_attack11_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_h01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackDash
    agent = "mewtwo", 
    script = "sound_attackdash_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackS3Hi
    agent = "mewtwo", 
    script = "sound_attacks3hi_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks3hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackS3
    agent = "mewtwo", 
    script = "sound_attacks3_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackS3Lw
    agent = "mewtwo", 
    script = "sound_attacks3lw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks3lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackHi3
    agent = "mewtwo", 
    script = "sound_attackhi3_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackLw3 
    agent = "mewtwo", 
    script = "sound_attacklw3_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_b01"));
    }
}

#[acmd_script(//AttackS4Hi
    agent = "mewtwo", 
    script = "sound_attacks4hi_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks4hi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4
    agent = "mewtwo", 
    script = "sound_attacks4_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackS4Lw
    agent = "mewtwo", 
    script = "sound_attacks4lw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacks4lw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackHi4
    agent = "mewtwo", 
    script = "sound_attackhi4_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack04"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_swing_s"));
    }
}

#[acmd_script(//AttackLw4 
    agent = "mewtwo", 
    script = "sound_attacklw4_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_attack05"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_n02"));
    }
}

#[acmd_script(//AttackAirN 
    agent = "mewtwo", 
    script = "sound_attackairn_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l02"));
        PLAY_SE(fighter, Hash40::new("se_silver_attackair_n02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackAirF 
    agent = "mewtwo", 
    script = "sound_attackairf_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_silver_attackair_f02"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n08"));
    }
}

#[acmd_script(//LandingAirF 
    agent = "mewtwo", 
    script = "sound_landingairf_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_landingairf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_mewtwo_final_01"));
        STOP_SE(fighter, Hash40::new("se_mewtwo_special_n08"));
    }
}

#[acmd_script(//AttackAirB 
    agent = "mewtwo", 
    script = "sound_attackairb_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_l02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackAirHi 
    agent = "mewtwo", 
    script = "sound_attackairhi_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attackair_f01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//AttackAirLw 
    agent = "mewtwo", 
    script = "sound_attackairlw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_throw_f02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_mewtwo_rnd_attack"));
    }
}

#[acmd_script(//ThrowF 
    agent = "mewtwo", 
    script = "sound_throwf_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_throwf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
    }
}

#[acmd_script(//ThrowB 
    agent = "mewtwo", 
    script = "sound_throwb_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_throwb(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowHi 
    agent = "mewtwo", 
    script = "sound_throwhi_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_throwhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//ThrowLw 
    agent = "mewtwo", 
    script = "sound_throwlw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_throwlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//CliffAttack 
    agent = "mewtwo", script = "sound_cliffattack_silver", category = ACMD_SOUND, low_priority )]
unsafe fn sound_silver_cliffattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

#[acmd_script(//DownAttackU 
    agent = "mewtwo", 
    script = "sound_downattacku_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_downattacku(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownAttackD 
    agent = "mewtwo", 
    script = "sound_downattackd_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_downattackd(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialNShoot, SpecialAirNShoot 
    agent = "mewtwo", 
    scripts = ["sound_specialnshoot_silver", "sound_specialairnshoot_silver"], 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialnshoot(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialSStart 
    agent = "mewtwo", 
    script = "sound_specialsstart_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialsstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100"));
    }
}

#[acmd_script(//SpecialS 
    agent = "mewtwo",
    script = "sound_specials_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specials(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_special_n01"));
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        let rand_sound = smash::app::sv_math::rand(hash40("mewtwo"), 6);
        if rand_sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s02"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s01"));
        }
    }
}

#[acmd_script(//SpecialAirSStart 
    agent = "mewtwo", 
    script = "sound_specialairsstart_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialairsstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_attack100"));
    }
}

#[acmd_script(//SpecialAirS 
    agent = "mewtwo",
    script = "sound_specialairs_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialairs(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_mewtwo_special_n01"));
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        let rand_sound = smash::app::sv_math::rand(hash40("mewtwo"), 6);
        if rand_sound == 0 {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s02"));
        }
        else {
            PLAY_SE(fighter, Hash40::new("vc_silver_special_s01"));
        }
    }
}

#[acmd_script(//SpecialHiStart 
    agent = "mewtwo", 
    script = "sound_specialhistart_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

#[acmd_script(//SpecialAirHiStart 
    agent = "mewtwo", 
    script = "sound_specialairhistart_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialairhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

#[acmd_script(//SpecialLw 
    agent = "mewtwo", 
    script = "sound_speciallw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_speciallw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_l01"));
    }
}

#[acmd_script(//SpecialAirLw 
    agent = "mewtwo", 
    script = "sound_specialairlw_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_specialairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_l01"));
    }
}

//Misc Scripts
#[acmd_script(//Run 
    agent = "mewtwo", 
    script = "sound_run_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_run(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//RunBrakeR 
    agent = "mewtwo", 
    script = "sound_runbraker_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_runbraker(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

#[acmd_script(//RunBrakeL 
    agent = "mewtwo", 
    script = "sound_runbrakel_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_runbrakel(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

#[acmd_script(//TurnRun 
    agent = "mewtwo", 
    script = "sound_turnrun_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_turnrun(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_dash_stop"));
    }
}

#[acmd_script(//EscapeAir 
    agent = "mewtwo", 
    script = "sound_escapeair_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_escapeair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape02"));
    }
}

#[acmd_script(//EscapeAirSlide 
    agent = "mewtwo", 
    script = "sound_escapeairslide_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_escapeairslide(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape02"));
    }
}

#[acmd_script(//EscapeN 
    agent = "mewtwo", 
    script = "sound_escapen_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_escapen(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_escape"));
    }
}

#[acmd_script(//EscapeF 
    agent = "mewtwo", 
    script = "sound_escapef_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_escapef(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
    }
}

#[acmd_script(//EscapeB 
    agent = "mewtwo", 
    script = "sound_escapeb_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_escapeb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
    }
}

#[acmd_script(//AppealHiL 
    agent = "mewtwo", 
    script = "sound_appealhil_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appealhil(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealHiR 
    agent = "mewtwo", 
    script = "sound_appealhir_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appealhir(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealSL 
    agent = "mewtwo", 
    script = "sound_appealsl_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_jump02"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_win3"));
    }
}

#[acmd_script(//AppealSR 
    agent = "mewtwo", 
    script = "sound_appealsr_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_jump02"));
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_mewtwo_win3"));
    }
}

#[acmd_script(//AppealLwL 
    agent = "mewtwo", 
    script = "sound_appeallwl_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appeallwl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

#[acmd_script(//AppealLwR 
    agent = "mewtwo",
    script = "sound_appeallwr_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_appeallwr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mewtwo_special_n07"));
    }
}

#[acmd_script(//Win1 
    agent = "mewtwo", 
    script = "sound_win1_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_mewtwo_escape"));
    }
}

#[acmd_script(//Win2 
    agent = "mewtwo", 
    script = "sound_win2_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_win2(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Win2 
    agent = "mewtwo", 
    script = "sound_win3_silver", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_silver_win3(fighter: &mut L2CAgentBase) {
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
    smashline::install_acmd_scripts!(
        sound_silver_attack11,
        sound_silver_attackdash,
        sound_silver_attacks3hi,
        sound_silver_attacks3,
        sound_silver_attacks3lw,
        sound_silver_attackhi3,
        sound_silver_attacklw3,
        sound_silver_attacks4hi,
        sound_silver_attacks4,
        sound_silver_attacks4lw,
        sound_silver_attackhi4,
        sound_silver_attacklw4,
        sound_silver_attackairn,
        sound_silver_attackairf,
        sound_silver_landingairf,
        sound_silver_attackairb,
        sound_silver_attackairhi,
        sound_silver_attackairlw,
        sound_silver_throwf,
        sound_silver_throwb,
        sound_silver_throwhi,
        sound_silver_throwlw,
        sound_silver_cliffattack,
        sound_silver_downattacku,
        sound_silver_downattackd,
        sound_silver_specialnshoot,
        sound_silver_specialsstart,
        sound_silver_specials,
        sound_silver_specialairsstart,
        sound_silver_specialairs,
        sound_silver_specialhistart,
        sound_silver_specialairhistart,
        sound_silver_speciallw,
        sound_silver_specialairlw,
        sound_silver_run,
        sound_silver_runbraker,
        sound_silver_runbrakel,
        sound_silver_turnrun,
        sound_silver_escapeair,
        sound_silver_escapeairslide,
        sound_silver_escapen,
        sound_silver_escapef,
        sound_silver_escapeb,
        sound_silver_appealhil,
        sound_silver_appealhir,
        sound_silver_appealsl,
        sound_silver_appealsr,
        sound_silver_appeallwl,
        sound_silver_appeallwr,
        sound_silver_win1,
        sound_silver_win2,
        sound_silver_win3
    );
}
