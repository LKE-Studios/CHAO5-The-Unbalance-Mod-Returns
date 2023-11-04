use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "dolly",
    script =  "sound_attack11_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_s"));
    }
}

#[acmd_script(//Attack12
    agent = "dolly",
    script =  "sound_attack12_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_m"));
    }
}

#[acmd_script(//Attack13
    agent = "dolly",
    script =  "sound_attack13_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attack13(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AttackDash
    agent = "dolly",
    script =  "sound_attackdash_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackdash01"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("seq_dolly_rnd_attackdash"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

#[acmd_script(//AttackS3 
    agent = "dolly", 
    script = "sound_attacks3_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_s01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

#[acmd_script(//AttackHi3 
    agent = "dolly", 
    script = "sound_attackhi3_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_escapeattack"));
    }
}

#[acmd_script(//AttackLw3
    agent = "dolly", 
    script = "sound_attacklw3_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_l01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
        SET_PLAY_INHIVIT(fighter, Hash40::new("se_dolly_squat"), 30);
    }
}

#[acmd_script(//AttackS4
    agent = "dolly",
    script =  "sound_attacks4_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_s01"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_s02"));
        PLAY_SE(fighter, Hash40::new("vc_dolly_attack07"));
    }
}

#[acmd_script(//AttackHi4 
    agent = "dolly", 
    script = "sound_attackhi4_waluigi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_waluigi_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_h01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_attack06"));
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_h02"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_h03"));
    }
}

#[acmd_script(//AttackLw4
    agent = "dolly",
    script =  "sound_attacklw4_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_kick_m"));
        PLAY_SE(fighter, Hash40::new("vc_dolly_attack05"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_punch_m"));
    }
}

#[acmd_script(//AttackAirN
    agent = "dolly",
    script =  "sound_attackairn_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_n01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
}

#[acmd_script(//AttackAirF
    agent = "dolly",
    script =  "sound_attackairf_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_f01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
}

#[acmd_script(//AttackAirB
    agent = "dolly",
    script =  "sound_attackairb_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_b01"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_b02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

#[acmd_script(//AttackAirHi
    agent = "dolly",
    script =  "sound_attackairhi_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {        
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

#[acmd_script(//AttackAirLw
    agent = "dolly",
    script =  "sound_attackairlw_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_l02"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_l01"));
    }
}

#[acmd_script(//CatchAttack 
    agent = "dolly", 
    script = "sound_catchattack_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_catchattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
}

#[acmd_script(//ThrowF
    agent = "dolly", 
    script = "sound_throwf_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_throwf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
}

#[acmd_script(//ThrowB 
    agent = "dolly", 
    script = "sound_throwb_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_throwb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_hit_kick_m"));
        PLAY_SE(fighter, Hash40::new("se_common_down_m_01"));
    }
}

#[acmd_script(//ThrowHi 
    agent = "dolly", 
    script = "sound_throwhi_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_throwhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_dolly_special_sb03_command"));
        PLAY_SE(fighter, Hash40::new("se_dolly_special_sb02_command"));
    }
}

#[acmd_script(//ThrowLw
    agent = "dolly",
    script =  "sound_throwlw_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_throwlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
}

#[acmd_script(//DownAttackD 
    agent = "dolly", 
    script = "sound_downattackd_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_downattackd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
}

#[acmd_script(//DownAttackU 
    agent = "dolly", 
    script = "sound_downattacku_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_downattacku(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
}    

#[acmd_script(//SpecialN
    agent = "dolly", 
    script = "sound_specialn_waluigi", 
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_ok"));
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
    }
}

#[acmd_script(//SpecialAirN
    agent = "dolly", 
    script = "sound_specialairn_waluigi", 
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 0.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_ok"));
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_dolly_superspecial_hit_critical"));
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
    }
}

#[acmd_script(//SpecialSBStart
    agent = "dolly",
    script =  "sound_specialsbstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialsbstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}

#[acmd_script(//SpecialSBAttack
    agent = "dolly",
    script =  "sound_specialsbattack_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialsbattack(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSBAttackW
    agent = "dolly",
    script =  "game_specialsbattackw_waluigi",
    category = ACMD_GAME, low_priority)]
unsafe fn sound_waluigi_specialsbattackw(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialAirSFEnd 
    agent = "dolly", 
    script = "sound_specialairsfend_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairsfend(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_water_hit_ll"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear_01"));
    }
}

#[acmd_script(//SpecialAirSFEnd2 
    agent = "dolly", 
    script = "sound_specialairsfend2",
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairsfend2(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSFStart
    agent = "dolly",
    script =  "sound_specialsfstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialsfstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}

#[acmd_script(//SpecialSFAttack 
    agent = "dolly", 
    script = "sound_specialsfattack_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialsfattack(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSFStart 
    agent = "dolly", 
    script = "sound_specialairsfstart_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairsfstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
    frame(fighter.lua_state_agent, 66.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_water_hit_m"));
    }
}

#[acmd_script(//SpecialAirSFAttack 
    agent = "dolly", 
    script = "sound_specialairsfattack_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairsfattack(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_dolly_special_sf03_command_l"));
            }
            else {
                if is_excute(fighter) {
                    PLAY_SE(fighter, Hash40::new("se_dolly_special_sf02_s"));
                }
                else {
                    if is_excute(fighter) {
                        PLAY_SE(fighter, Hash40::new("se_dolly_special_sf02_l"));
                    }
                }
            }
        }
        else {
            if is_excute(fighter) {
                PLAY_SE(fighter, Hash40::new("se_dolly_special_sf03_command_s"));
            }
        }
    }
}

#[acmd_script(//SpecialHi1 
    agent = "dolly", 
    script = "sound_specialhi1_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_s"));
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_s"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h01"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_s"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_s"));
    }
    else {
        frame(fighter.lua_state_agent, 0.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_l"));
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_l"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h02"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_l"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_l"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_l"));
    }
}

#[acmd_script(//SpecialAirHi1 
    agent = "dolly", 
    script = "sound_specialairhi1_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairhi1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_s"));
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_s"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h01"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_s"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_s"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_s"));
    }
    else {
        frame(fighter.lua_state_agent, 0.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h01_l"));
            PLAY_SE(fighter, Hash40::new("se_dolly_special_h02_l"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_h02"));
        }
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h03_l"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h04_l"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h05_l"));
    }
}

#[acmd_script(//SpecialAirLw 
    agent = "dolly", 
    script = "sound_specialairlw_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_specialairlw(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialLwStart
    agent = "dolly",
    script =  "sound_speciallwstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "dolly",
    script =  "sound_specialairlwstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}

#[acmd_script(//SpecialLwShield
    agent = "dolly",
    script =  "sound_speciallwshield",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwshield(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_system_fp_level_up"));
    }
}

#[acmd_script(//SpecialLwAttack1
    agent = "dolly",
    script =  "sound_speciallwattack1",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattack1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_l01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

#[acmd_script(//SpecialLwAttack2
    agent = "dolly",
    script =  "sound_speciallwattack2",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattack2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_l02"));
    }	
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_s01"));
    }
}

#[acmd_script(//SpecialLwAttack3
    agent = "dolly",
    script =  "sound_speciallwattack3",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattack3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_h02"));
    }	
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_n05"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

#[acmd_script(//SpecialLwAttackSpecial1
    agent = "dolly",
    script =  "sound_speciallwattackspecial1",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattackspecial1(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

#[acmd_script(//SpecialLwAttackSpecial2
    agent = "dolly",
    script =  "sound_speciallwattackspecial2",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattackspecial2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_b01"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_b02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

#[acmd_script(//SpecialLwJump
    agent = "dolly",
    script =  "sound_speciallwjump",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwjump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
}

#[acmd_script(//SpecialLwSpecial
    agent = "dolly",
    script =  "sound_speciallwspecial",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {        
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

#[acmd_script(//SpecialLwAttackAir
    agent = "dolly",
    script =  "sound_speciallwattackair",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwattackair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_l01"));
    }
	wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

#[acmd_script(//SpecialLwSpecialAir
    agent = "dolly",
    script =  "sound_speciallwspecialair",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwspecialair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackdash01"));
    }
}

#[acmd_script(//SpecialLwJumpAir
    agent = "dolly",
    script =  "sound_speciallwjumpair",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_speciallwjumpair(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
    }
}

#[acmd_script(//SuperSpecial 
    agent = "dolly", 
    script = "sound_superspecial_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_superspecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_special_ss01"));
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial01_01"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial01_02"));
    }
}

#[acmd_script(//SuperSpecial2 
    agent = "dolly", 
    script = "sound_superspecial2_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_superspecial2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_04"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_superspecial02_02"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_05"));
    }
}

#[acmd_script(//SuperSpecial2Start 
    agent = "dolly", 
    script = "sound_superspecial2start_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_superspecial2start(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial_success"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_superspecial02_01"));
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_03"));
    }
}

#[acmd_script(//AppealHiL 
    agent = "dolly", 
    script = "sound_appealhil_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x136a368b1c));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_appeal01_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
}

#[acmd_script(//AppealHiR
    agent = "dolly", 
    script = "sound_appealhir_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x136a368b1c));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_appeal01_02"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new_raw(0x13f33fdaa6));
    }
}

#[acmd_script(//AppealLwL 
    agent = "dolly", 
    script = "sound_appeallwl_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_appeallwl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_win02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l03"));
    }
}

#[acmd_script(//AppealLwR 
    agent = "dolly", 
    script = "sound_appeallwr_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_appeallwr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_dolly_win02"));
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l01"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l02"));
    }
    frame(fighter.lua_state_agent, 85.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_appeal_l03"));
    }
}

#[acmd_script(//EntryR 
    agent = "dolly", 
    script = "sound_entryr_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_entryr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear01"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear02"));
    }
    frame(fighter.lua_state_agent, 84.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear03"));
    }
}

#[acmd_script(//EntryL
    agent = "dolly", 
    script = "sound_entryl_waluigi", 
    category = ACMD_SOUND, low_priority )]
unsafe fn sound_waluigi_entryl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear01"));
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear02"));
    }
    frame(fighter.lua_state_agent, 84.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_appear03"));
    }
}

#[acmd_script(//FinalStart
    agent = "dolly",
    script =  "sound_finalstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_finalstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final01"));
	}
		frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final06"));
	}
}

#[acmd_script(//FinalAirStart
    agent = "dolly",
    script =  "sound_finalairstart_waluigi",
    category = ACMD_SOUND, low_priority)]
unsafe fn sound_waluigi_finalairstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final01"));
	}
		frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final06"));
	}
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_waluigi_attack11,
        sound_waluigi_attack12,
        sound_waluigi_attack13,
        sound_waluigi_attackdash,
        sound_waluigi_attacks3,
        sound_waluigi_attackhi3,
        sound_waluigi_attacklw3,
        sound_waluigi_attacks4,
        sound_waluigi_attackhi4,
        sound_waluigi_attacklw4,
        sound_waluigi_attackairn,
        sound_waluigi_attackairf,
        sound_waluigi_attackairb,
        sound_waluigi_attackairhi,
        sound_waluigi_attackairlw,
        sound_waluigi_catchattack,
        sound_waluigi_throwf,
        sound_waluigi_throwb,
        sound_waluigi_throwhi,
        sound_waluigi_throwlw,
        sound_waluigi_downattacku,
        sound_waluigi_downattackd,
        sound_waluigi_specialn,
        sound_waluigi_specialairn,
        sound_waluigi_specialsbstart,
        sound_waluigi_specialsbattackw,
        sound_waluigi_specialsbattack,
        sound_waluigi_specialairsfend,
        sound_waluigi_specialairsfend2,
        sound_waluigi_specialsfstart,
        sound_waluigi_specialairsfstart,
        sound_waluigi_specialsfattack,
        sound_waluigi_specialairsfattack,
        sound_waluigi_specialhi1,
        sound_waluigi_specialairhi1,
        sound_waluigi_speciallwstart,
        sound_waluigi_specialairlwstart,
        sound_waluigi_specialairlw,
        sound_waluigi_speciallwshield,
        sound_waluigi_speciallwattack1,
        sound_waluigi_speciallwattack2,
        sound_waluigi_speciallwattack3,
        sound_waluigi_speciallwattackspecial1,
        sound_waluigi_speciallwattackspecial2,
        sound_waluigi_speciallwjump,
        sound_waluigi_speciallwattackair,
        sound_waluigi_speciallwspecialair,
        sound_waluigi_speciallwspecial,
        sound_waluigi_speciallwjumpair,
        sound_waluigi_superspecial,
        sound_waluigi_superspecial2,
        sound_waluigi_superspecial2start,
        sound_waluigi_appealhil,
        sound_waluigi_appealhir,
        sound_waluigi_appeallwl,
        sound_waluigi_appeallwr,
        sound_waluigi_entryl,
        sound_waluigi_entryr,
        sound_waluigi_finalstart,
        sound_waluigi_finalairstart
    );
}