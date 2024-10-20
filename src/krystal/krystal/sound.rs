use crate::imports::BuildImports::*; 

//Attack11 
unsafe extern "C" fn sound_krystal_Attack11(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_swing_s"));
	}
}

//Attack12
unsafe extern "C" fn sound_krystal_Attack12(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 5.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
	}
}

//Attack13
unsafe extern "C" fn sound_krystal_Attack13(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
		PLAY_SE(fighter, Hash40::new("se_pitb_swing_l"));
	}
}

//AttackDash
unsafe extern "C" fn sound_krystal_AttackDash(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
		PLAY_STATUS(fighter, Hash40::new("se_pitb_attack100"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
	frame(fighter.lua_state_agent, 34.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_s"));
	}
}

//AttackS3
unsafe extern "C" fn sound_krystal_AttackS3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_n01"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
}

//AttackHi3
unsafe extern "C" fn sound_krystal_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
		PLAY_SE(fighter, Hash40::new("se_pitb_swing_s"));
	}
	frame(fighter.lua_state_agent, 15.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
	}
}

//AttackLw3
unsafe extern "C" fn sound_krystal_AttackLw3(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 7.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_swing_03"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
		PLAY_SE(fighter, Hash40::new("se_pitb_swing_s"));
	}
}

//AttackS4Charge
unsafe extern "C" fn sound_krystal_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_03"));
    }
}

//AttackS4
unsafe extern "C" fn sound_krystal_AttackS4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
	}
	frame(fighter.lua_state_agent, 13.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_pitb_attack05"));
		PLAY_SE(fighter, Hash40::new("se_pitb_smash_s01"));
	}
	frame(fighter.lua_state_agent, 14.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_sword_sword_m"));
	}
}

//AttackHi4Charge
unsafe extern "C" fn sound_krystal_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_03"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_krystal_AttackHi4(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
	}
	frame(fighter.lua_state_agent, 7.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
	}
	frame(fighter.lua_state_agent, 8.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_pitb_attack06"));
	}
	frame(fighter.lua_state_agent, 35.0);
	if is_excute(fighter) {
		PLAY_LANDING_SE(fighter, Hash40::new("se_pitb_landing01"));
	}
}

//AttackLw4Charge
unsafe extern "C" fn sound_krystal_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_03"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_krystal_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
	if is_excute(fighter) {
		STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
	}
	wait(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_pitb_attack07"));
	}
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_LW4_IS_CHARGED) { 
	    frame(fighter.lua_state_agent, 23.0);
	    if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_l"));
		}
    }
	else {
        frame(fighter.lua_state_agent, 23.0);
	    if is_excute(fighter) {
			PLAY_SE(fighter, Hash40::new("se_pitb_smash_l01"));
		}
	}
	frame(fighter.lua_state_agent, 24.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_common_heavy_hit_m"));
	}
	wait(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		PLAY_LANDING_SE(fighter, Hash40::new("se_pitb_landing01"));
	}
}

//AttackAirN
unsafe extern "C" fn sound_krystal_AttackAirN(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_n01"));
	}
}

//AttackAirF 
unsafe extern "C" fn sound_krystal_AttackAirF(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 7.0);
	if is_excute(fighter) {
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_n01"));
		
	}
	frame(fighter.lua_state_agent, 15.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_f01"));
	}
}

//AttackAirB
unsafe extern "C" fn sound_krystal_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_pitb_attackair_n01"));
    }
}

//AttackAirHi
unsafe extern "C" fn sound_krystal_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_07"));
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_krystal_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_06"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_l"));
    }
}

//ThrowF
unsafe extern "C" fn sound_krystal_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
    }
}

//ThrowB
unsafe extern "C" fn sound_krystal_ThrowB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
    }
}

//ThrowHi
unsafe extern "C" fn sound_krystal_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_01"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_attackair_h01"));
    }
}      

//ThrowLw
unsafe extern "C" fn sound_krystal_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_down_soil_s"));
        PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_special_s02"));
    }
}

//SlipAttack
unsafe extern "C" fn sound_krystal_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_s"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_s"));
    }
}

//SpecialNFireS
unsafe extern "C" fn sound_krystal_SpecialNFireS(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		STOP_SE(fighter, Hash40::new("se_pitb_special_n01"));
	}
	wait(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_n02"));
		SET_PLAY_INHIVIT(fighter, Hash40::new("se_pitb_special_n02"), 30);
	}
	frame(fighter.lua_state_agent, 20.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_h01"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
}

//SpecialAirNFireS
unsafe extern "C" fn sound_krystal_SpecialAirNFireS(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 2.0);
	if is_excute(fighter) {
		STOP_SE(fighter, Hash40::new("se_pitb_special_n01"));
	}
	wait(fighter.lua_state_agent, 4.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_n02"));
		SET_PLAY_INHIVIT(fighter, Hash40::new("se_pitb_special_n02"), 30);
	}
	frame(fighter.lua_state_agent, 20.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_attackair_h01"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
}

//SpecialSStart
unsafe extern "C" fn sound_krystal_SpecialSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s01"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
	frame(fighter.lua_state_agent, 14.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s02"));
	}
}

//SpecialAirSStart
unsafe extern "C" fn sound_krystal_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s01"));
		PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_attack"));
	}
	frame(fighter.lua_state_agent, 14.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s02"));
	}
}

//SpecialHi
unsafe extern "C" fn sound_krystal_SpecialHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_special_h02"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_pitb_rnd_special_h"));
    }
}

//SpecialHiStart
unsafe extern "C" fn sound_krystal_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_special_h01"));
    }
}

//SpecialAirHiStart
unsafe extern "C" fn sound_krystal_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_special_h01"));
    }
}

//SpecialLwHold
unsafe extern "C" fn sound_krystal_SpecialLwHold(fighter: &mut L2CAgentBase) {
	frame(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 66.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 131.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 196.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 261.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 326.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 380.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 440.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
}

//SpecialAirLwHold
unsafe extern "C" fn sound_krystal_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 66.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 131.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 196.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 261.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 326.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 380.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
	frame(fighter.lua_state_agent, 440.0);
	if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("se_pitb_special_s04"));
	}
}

//AppealLwL
unsafe extern "C" fn sound_krystal_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_pitb_appeal03"));
	}
	frame(fighter.lua_state_agent, 98.0);
	if is_excute(fighter) {
		PLAY_STEP(fighter, Hash40::new("se_pitb_landing01"));
	}
}

//AppealLwR
unsafe extern "C" fn sound_krystal_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_final02"));
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_rise_win01"));
    }
    frame(fighter.lua_state_agent, 80.0);
    if is_excute(fighter) {
		PLAY_SE(fighter, Hash40::new("vc_pitb_appeal03"));
	}
	frame(fighter.lua_state_agent, 98.0);
	if is_excute(fighter) {
		PLAY_STEP(fighter, Hash40::new("se_pitb_landing01"));
	}
}

//Final
unsafe extern "C" fn sound_krystal_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_pitb_final02"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_final02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_pitb_final03"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_pitb_final04"));
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_pitb_final03"));
        STOP_SE(fighter, Hash40::new("se_pitb_final04"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
    }
}

//FinalAir
unsafe extern "C" fn sound_krystal_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_pitb_final01"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_final02"));
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_pitb_final03"));
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_pitb_final04"));
    }
    frame(fighter.lua_state_agent, 83.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_pitb_final03"));
        STOP_SE(fighter, Hash40::new("se_pitb_final04"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_m"));
        PLAY_SE(fighter, Hash40::new("se_common_bomb_ll"));
    }
}

//Win1
unsafe extern "C" fn sound_krystal_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        let entry_count = FighterManager::entry_count(FighterManager());
        let mut found_fox = false;
        for i in 0..entry_count {
            let chara = the_csk_collection_api::get_ui_chara_from_entry_id(i as u32);
            if chara == hash40("ui_chara_fox") {
                found_fox = true;
                break;
            }
        }
        if found_fox {
            PLAY_SE_NO_3D(fighter, Hash40::new("vc_pitb_win03_02"));
        } 
		else {
            PLAY_SE_NO_3D(fighter, Hash40::new("vc_pitb_win01"));
        }
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_m_win01"));
    }
    frame(fighter.lua_state_agent, 92.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_win1_win01"));
    }
}

//Win3
unsafe extern "C" fn sound_krystal_Win3(fighter: &mut L2CAgentBase) {}

//Win3a
unsafe extern "C" fn sound_krystal_Win3a(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_pitb_win03"));
    }
    frame(fighter.lua_state_agent, 123.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
}

//Win3b
unsafe extern "C" fn sound_krystal_Win3b(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_pitb_win03"));
    }
    frame(fighter.lua_state_agent, 123.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_04"));
    }
}

pub fn install() {
    Agent::new("pitb")
    .sound_acmd("sound_attack11_krystal", sound_krystal_Attack11, Low)
    .sound_acmd("sound_attack12_krystal", sound_krystal_Attack12, Low)
    .sound_acmd("sound_attack13_krystal", sound_krystal_Attack13, Low)
    .sound_acmd("sound_attackdash_krystal", sound_krystal_AttackDash, Low)
    .sound_acmd("sound_attacks3_krystal", sound_krystal_AttackS3, Low)
    .sound_acmd("sound_attackhi3_krystal", sound_krystal_AttackHi3, Low)
    .sound_acmd("sound_attacklw3_krystal", sound_krystal_AttackLw3, Low)
    .sound_acmd("sound_attacks4_krystal", sound_krystal_AttackS4, Low)
    .sound_acmd("sound_attackhi4_krystal", sound_krystal_AttackHi4, Low)
    .sound_acmd("sound_attacklw4_krystal", sound_krystal_AttackLw4, Low)
	.sound_acmd("sound_attacks4charge_krystal", sound_krystal_AttackS4Charge, Low)
    .sound_acmd("sound_attackhi4charge_krystal", sound_krystal_AttackHi4Charge, Low)
    .sound_acmd("sound_attacklw4charge_krystal", sound_krystal_AttackLw4Charge, Low)
    .sound_acmd("sound_attackairn_krystal", sound_krystal_AttackAirN, Low)
    .sound_acmd("sound_attackairf_krystal", sound_krystal_AttackAirF, Low)    
    .sound_acmd("sound_attackairb_krystal", sound_krystal_AttackAirB, Low)
    .sound_acmd("sound_attackairhi_krystal", sound_krystal_AttackAirHi, Low)
    .sound_acmd("sound_attackairlw_krystal", sound_krystal_AttackAirLw, Low)
    .sound_acmd("sound_throwf_krystal", sound_krystal_ThrowF, Low)
    .sound_acmd("sound_throwb_krystal", sound_krystal_ThrowB, Low)
    .sound_acmd("sound_throwhi_krystal", sound_krystal_ThrowHi, Low)
    .sound_acmd("sound_throwlw_krystal", sound_krystal_ThrowLw, Low)
    .sound_acmd("sound_slipattack_krystal", sound_krystal_SlipAttack, Low)
    .sound_acmd("sound_specialnfires_krystal", sound_krystal_SpecialNFireS, Low)  
    .sound_acmd("sound_specialairnfires_krystal", sound_krystal_SpecialAirNFireS, Low)  
    .sound_acmd("sound_specialsstart_krystal", sound_krystal_SpecialSStart, Low)  
    .sound_acmd("sound_specialairsstart_krystal", sound_krystal_SpecialAirSStart, Low)  
    .sound_acmd("sound_specialhi_krystal", sound_krystal_SpecialHi, Low)
    .sound_acmd("sound_specialhistart_krystal", sound_krystal_SpecialHiStart, Low)
    .sound_acmd("sound_specialairhistart_krystal", sound_krystal_SpecialAirHiStart, Low)
    .sound_acmd("sound_speciallwhold_krystal", sound_krystal_SpecialLwHold, Low)
    .sound_acmd("sound_specialairlwhold_krystal", sound_krystal_SpecialAirLwHold, Low)
    .sound_acmd("sound_appeallwl_krystal", sound_krystal_AppealLwL, Low)
    .sound_acmd("sound_appeallwr_krystal", sound_krystal_AppealLwR, Low)
    .sound_acmd("sound_final_krystal", sound_krystal_Final, Low)
    .sound_acmd("sound_finalair_krystal", sound_krystal_FinalAir, Low)
	.sound_acmd("sound_win1_krystal", sound_krystal_Win1, Low)
    .sound_acmd("sound_win3_krystal", sound_krystal_Win3, Low)
    .sound_acmd("sound_win3a_krystal", sound_krystal_Win3a, Low)
    .sound_acmd("sound_win3b_krystal", sound_krystal_Win3b, Low)
    .install();
}