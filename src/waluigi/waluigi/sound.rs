use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn sound_waluigi_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_s"));
    }
}

//Attack12
unsafe extern "C" fn sound_waluigi_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_swing_m"));
    }
}

//Attack13
unsafe extern "C" fn sound_waluigi_Attack13(fighter: &mut L2CAgentBase) {}

//AttackDash
unsafe extern "C" fn sound_waluigi_AttackDash(fighter: &mut L2CAgentBase) {
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

//AttackS3 
unsafe extern "C" fn sound_waluigi_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_s01"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

//AttackHi3 
unsafe extern "C" fn sound_waluigi_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_escapeattack"));
    }
}

//AttackLw3
unsafe extern "C" fn sound_waluigi_AttackLw3(fighter: &mut L2CAgentBase) {
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

//AttackS4
unsafe extern "C" fn sound_waluigi_AttackS4(fighter: &mut L2CAgentBase) {
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

//AttackHi4 
unsafe extern "C" fn sound_waluigi_AttackHi4(fighter: &mut L2CAgentBase) {
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

//AttackLw4
unsafe extern "C" fn sound_waluigi_AttackLw4(fighter: &mut L2CAgentBase) {
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

//AttackAirN
unsafe extern "C" fn sound_waluigi_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_n01"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_waluigi_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
    }
    wait(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_f01"));
        PLAY_SE(fighter, Hash40::new("seq_dolly_rnd_attack_s"));
    }
}

//AttackAirB
unsafe extern "C" fn sound_waluigi_AttackAirB(fighter: &mut L2CAgentBase) {
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

//AttackAirHi
unsafe extern "C" fn sound_waluigi_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_l01"));
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {        
        PLAY_STATUS(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_waluigi_AttackAirLw(fighter: &mut L2CAgentBase) {
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

//CatchAttack 
unsafe extern "C" fn sound_waluigi_CatchAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_h09_command_s"));
    }
}

//ThrowF
unsafe extern "C" fn sound_waluigi_ThrowF(fighter: &mut L2CAgentBase) {
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
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//ThrowB 
unsafe extern "C" fn sound_waluigi_ThrowB(fighter: &mut L2CAgentBase) {
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

//ThrowHi 
unsafe extern "C" fn sound_waluigi_ThrowHi(fighter: &mut L2CAgentBase) {
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

//ThrowLw
unsafe extern "C" fn sound_waluigi_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_dolly_special_h01"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
}

//DownAttackD 
unsafe extern "C" fn sound_waluigi_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
}

//DownAttackU 
unsafe extern "C" fn sound_waluigi_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_fire_m_damage"));
    }
}    

//SpecialN
unsafe extern "C" fn sound_waluigi_SpecialN(fighter: &mut L2CAgentBase) {
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
}

//SpecialAirN
unsafe extern "C" fn sound_waluigi_SpecialAirN(fighter: &mut L2CAgentBase) {
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
}

//SpecialSBStart
unsafe extern "C" fn sound_waluigi_SpecialSBStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}

//SpecialSBAttack
unsafe extern "C" fn sound_waluigi_SpecialSBAttack(fighter: &mut L2CAgentBase) {}

//SpecialSBAttackW
unsafe extern "C" fn sound_waluigi_SpecialSBAttackW(fighter: &mut L2CAgentBase) {}

//SpecialAirSFEnd 
unsafe extern "C" fn sound_waluigi_SpecialAirSFEnd(fighter: &mut L2CAgentBase) {
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

//SpecialAirSFEnd2 
unsafe extern "C" fn sound_waluigi_SpecialAirSFEnd2(fighter: &mut L2CAgentBase) {}

//SpecialSFStart
unsafe extern "C" fn sound_waluigi_SpecialSFStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_superspecial02_catch"));
    }
}

//SpecialSFAttack 
unsafe extern "C" fn sound_waluigi_SpecialSFAttack(fighter: &mut L2CAgentBase) {}

//SpecialAirSFStart 
unsafe extern "C" fn sound_waluigi_SpecialAirSFStart(fighter: &mut L2CAgentBase) {
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

//SpecialAirSFAttack 
unsafe extern "C" fn sound_waluigi_SpecialAirSFAttack(fighter: &mut L2CAgentBase) {
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

//SpecialHi1 
unsafe extern "C" fn sound_waluigi_SpecialHi1(fighter: &mut L2CAgentBase) {
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

//SpecialAirHi1 
unsafe extern "C" fn sound_waluigi_SpecialAirHi1(fighter: &mut L2CAgentBase) {
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

//SpecialAirLw 
unsafe extern "C" fn sound_waluigi_SpecialAirLw(fighter: &mut L2CAgentBase) {}

//SpecialLwStart
unsafe extern "C" fn sound_waluigi_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_waluigi_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_win02_02"));
    }
}

//SpecialLwShield
unsafe extern "C" fn sound_waluigi_SpecialLwShield(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_system_fp_level_up"));
    }
}

//SpecialLwAttack1
unsafe extern "C" fn sound_waluigi_SpecialLwAttack1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_l01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

//SpecialLwAttack2
unsafe extern "C" fn sound_waluigi_SpecialLwAttack2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_smash_l02"));
    }	
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_s01"));
    }
}

//SpecialLwAttack3
unsafe extern "C" fn sound_waluigi_SpecialLwAttack3(fighter: &mut L2CAgentBase) {
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

//SpecialLwAttackSpecial1
unsafe extern "C" fn sound_waluigi_SpecialLwAttackSpecial1(fighter: &mut L2CAgentBase) {
        frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_special_n02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attack_l"));
    }
}

//SpecialLwAttackSpecial2
unsafe extern "C" fn sound_waluigi_SpecialLwAttackSpecial2(fighter: &mut L2CAgentBase) {
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

//SpecialLwJump
unsafe extern "C" fn sound_waluigi_SpecialLwJump(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_jump01"));
    }
}

//SpecialLwSpecial
unsafe extern "C" fn sound_waluigi_SpecialLwSpecial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {        
        PLAY_SE(fighter, Hash40::new("se_dolly_attackair_h01"));
    }
}

//SpecialLwAttackAir
unsafe extern "C" fn sound_waluigi_SpecialLwAttackAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_l01"));
    }
	wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackhard_h01"));
    }
}

//SpecialLwSpecialAir
unsafe extern "C" fn sound_waluigi_SpecialLwSpecialAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_attackdash01"));
    }
}

//SpecialLwJumpAir
unsafe extern "C" fn sound_waluigi_SpecialLwJumpAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_waluigi_special_l01_jump_air")); //Index 140
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_stage_mariobros_09"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_waluigi_special_l02_jump_air")); //Index 141
    }
}

//SuperSpecial 
unsafe extern "C" fn sound_waluigi_SuperSpecial(fighter: &mut L2CAgentBase) {
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

//SuperSpecial2 
unsafe extern "C" fn sound_waluigi_SuperSpecial2(fighter: &mut L2CAgentBase) {
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

//SuperSpecial2Start 
unsafe extern "C" fn sound_waluigi_SuperSpecial2Start(fighter: &mut L2CAgentBase) {
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

//AppealHiL 
unsafe extern "C" fn sound_waluigi_AppealHiL(fighter: &mut L2CAgentBase) {
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

//AppealHiR
unsafe extern "C" fn sound_waluigi_AppealHiR(fighter: &mut L2CAgentBase) {
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

//AppealLwL 
unsafe extern "C" fn sound_waluigi_AppealLwL(fighter: &mut L2CAgentBase) {
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

//AppealLwR 
unsafe extern "C" fn sound_waluigi_AppealLwR(fighter: &mut L2CAgentBase) {
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

//EntryR 
unsafe extern "C" fn sound_waluigi_EntryR(fighter: &mut L2CAgentBase) {
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

//EntryL
unsafe extern "C" fn sound_waluigi_EntryL(fighter: &mut L2CAgentBase) {
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

//FinalStart
unsafe extern "C" fn sound_waluigi_FinalStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final01"));
	}
		frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dolly_final06"));
	}
}

//FinalAirStart
unsafe extern "C" fn sound_waluigi_FinalAirStart(fighter: &mut L2CAgentBase) {
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
    Agent::new("dolly")
    .sound_acmd("sound_attack11_waluigi", sound_waluigi_Attack11)
    .sound_acmd("sound_attack12_waluigi", sound_waluigi_Attack12)
    .sound_acmd("sound_attack13_waluigi", sound_waluigi_Attack13)
    .sound_acmd("sound_attackdash_waluigi", sound_waluigi_AttackDash)
    .sound_acmd("sound_attacks3_waluigi", sound_waluigi_AttackS3)
    .sound_acmd("sound_attackhi3_waluigi", sound_waluigi_AttackHi3)
    .sound_acmd("sound_attacklw3_waluigi", sound_waluigi_AttackLw3)
    .sound_acmd("sound_attacks4_waluigi", sound_waluigi_AttackS4)
    .sound_acmd("sound_attackhi4_waluigi", sound_waluigi_AttackHi4)
    .sound_acmd("sound_attacklw4_waluigi", sound_waluigi_AttackLw4)
    .sound_acmd("sound_attackairn_waluigi", sound_waluigi_AttackAirN)
    .sound_acmd("sound_attackairf_waluigi", sound_waluigi_AttackAirF)
    .sound_acmd("sound_attackairb_waluigi", sound_waluigi_AttackAirB)
    .sound_acmd("sound_attackairhi_waluigi", sound_waluigi_AttackAirHi)
    .sound_acmd("sound_attackairlw_waluigi", sound_waluigi_AttackAirLw)
    .sound_acmd("sound_catchattack_waluigi", sound_waluigi_CatchAttack)
    .sound_acmd("sound_throwf_waluigi", sound_waluigi_ThrowF)
    .sound_acmd("sound_throwb_waluigi", sound_waluigi_ThrowB)
    .sound_acmd("sound_throwhi_waluigi", sound_waluigi_ThrowHi)
    .sound_acmd("sound_throwlw_waluigi", sound_waluigi_ThrowLw)
    .sound_acmd("sound_downattacku_waluigi", sound_waluigi_DownAttackU)
    .sound_acmd("sound_downattackd_waluigi", sound_waluigi_DownAttackD)
    .sound_acmd("sound_specialn_waluigi", sound_waluigi_SpecialN)
    .sound_acmd("sound_specialairn_waluigi", sound_waluigi_SpecialAirN)
    .sound_acmd("sound_specialsbstart_waluigi", sound_waluigi_SpecialSBStart)
    .sound_acmd("sound_specialsbattackw_waluigi", sound_waluigi_SpecialSBAttackW)
    .sound_acmd("sound_specialsbattack_waluigi", sound_waluigi_SpecialSBAttack)
    .sound_acmd("sound_specialairsfend_waluigi", sound_waluigi_SpecialAirSFEnd)
    .sound_acmd("sound_specialairsfend2", sound_waluigi_SpecialAirSFEnd2)
    .sound_acmd("sound_specialsfattack_waluigi", sound_waluigi_SpecialSFAttack)
    .sound_acmd("sound_specialsfstart_waluigi", sound_waluigi_SpecialSFStart)
    .sound_acmd("sound_specialairsfstart_waluigi", sound_waluigi_SpecialAirSFStart)
    .sound_acmd("sound_specialhi1_waluigi", sound_waluigi_SpecialHi1)
    .sound_acmd("sound_specialairhi1_waluigi", sound_waluigi_SpecialAirHi1)
    .sound_acmd("sound_speciallwstart_waluigi", sound_waluigi_SpecialLwStart)
    .sound_acmd("sound_specialairlwstart_waluigi", sound_waluigi_SpecialAirLwStart)
    .sound_acmd("sound_specialairlw_waluigi", sound_waluigi_SpecialAirLw)
    .sound_acmd("sound_speciallwshield", sound_waluigi_SpecialLwShield)
    .sound_acmd("sound_speciallwattack1", sound_waluigi_SpecialLwAttack1)
    .sound_acmd("sound_speciallwattack2", sound_waluigi_SpecialLwAttack2)
    .sound_acmd("sound_speciallwattack3", sound_waluigi_SpecialLwAttack3)
    .sound_acmd("sound_speciallwattackspecial1", sound_waluigi_SpecialLwAttackSpecial1)
    .sound_acmd("sound_speciallwattackspecial2", sound_waluigi_SpecialLwAttackSpecial2)
    .sound_acmd("sound_speciallwjump", sound_waluigi_SpecialLwJump)
    .sound_acmd("sound_speciallwattackair", sound_waluigi_SpecialLwAttackAir)
    .sound_acmd("sound_speciallwspecialair", sound_waluigi_SpecialLwSpecialAir)
    .sound_acmd("sound_speciallwspecial", sound_waluigi_SpecialLwSpecial)
    .sound_acmd("sound_speciallwjumpair", sound_waluigi_SpecialLwJumpAir)
    .sound_acmd("sound_superspecial_waluigi", sound_waluigi_SuperSpecial)
    .sound_acmd("sound_superspecial2_waluigi", sound_waluigi_SuperSpecial2)
    .sound_acmd("sound_superspecial2start_waluigi", sound_waluigi_SuperSpecial2Start)
    .sound_acmd("sound_finalstart_waluigi", sound_waluigi_FinalStart)
    .sound_acmd("sound_finalairstart_waluigi", sound_waluigi_FinalAirStart)
    .sound_acmd("sound_entryl_waluigi", sound_waluigi_EntryL)
    .sound_acmd("sound_entryr_waluigi", sound_waluigi_EntryR)
    .install();
}