use crate::imports::BuildImports::*;

//Run
unsafe extern "C" fn sound_koopag_Run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopag_run_loop"));
    }
}

//SpecialNStart
unsafe extern "C" fn sound_koopag_SpecialNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_step_left_m"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

//SpecialAirNStart
unsafe extern "C" fn sound_koopag_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

//SpecialSCatch
unsafe extern "C" fn sound_koopag_SpecialSCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_koopag_nailswing01"));
    }
}

//SpecialSAirCatch
unsafe extern "C" fn sound_koopag_SpecialSAirCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_s01"));
        PLAY_SE(fighter, Hash40::new("se_koopag_nailswing01"));
    }	
}

//DownBoundD
unsafe extern "C" fn sound_koopag_DownBoundD(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

//DownBoundU
unsafe extern "C" fn sound_koopag_DownBoundU(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

//DownAttackD
unsafe extern "C" fn sound_koopag_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_rise"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_nailswing02"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_left_m"));
    }
}

//DownAttackU
unsafe extern "C" fn sound_koopag_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_rise"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_nailswing02"));
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_left_m"));
    }
}

//AppealHiL
unsafe extern "C" fn sound_koopag_AppealHiL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopag_attack07"));
    }
}

//AppealHiR
unsafe extern "C" fn sound_koopag_AppealHiR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("vc_koopag_attack07"));
    }
}

//AppealLwL
unsafe extern "C" fn sound_koopag_AppealLwL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
        PLAY_SE(fighter, Hash40::new("vc_koopag_ottotto"));
    }
    wait(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
    }
    wait(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
    }
}

//AppealLwR
unsafe extern "C" fn sound_koopag_AppealLwR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
        PLAY_SE(fighter, Hash40::new("vc_koopag_ottotto"));
    }
    wait(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
    }
    wait(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_step_right_m"));
    }
}

//AppealSL
unsafe extern "C" fn sound_koopag_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_m"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_s"));
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_m"));
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 86.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_s"));
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    } 
}

//AppealSR
unsafe extern "C" fn sound_koopag_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_m"));
    }
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_s"));
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_m"));
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }
    frame(fighter.lua_state_agent, 86.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_swing_s"));
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_koopag_appeal02"));
    }   
}

//Damage
unsafe extern "C" fn sound_koopag_Damage(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGE_VC(fighter);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_koopag_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_koopag_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyN
unsafe extern "C" fn sound_koopag_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_koopag_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_koopag_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//ShieldBreakFly
unsafe extern "C" fn sound_koopag_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DAMAGEFLY_VC(fighter);
    }
}

//FuraFuraStartD
unsafe extern "C" fn sound_koopag_FuraFuraStartD(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FURAFURA_VOICE_FINAL) {
        if is_excute(fighter) {
            SET_PLAY_INHIVIT(fighter, Hash40::new("se_common_dizzy"), 10);
        }
    }
    else {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_common_dizzy"));
        }
    }
}

//FuraFuraStartU
unsafe extern "C" fn sound_koopag_FuraFuraStartU(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FURAFURA_VOICE_FINAL) {
        if is_excute(fighter) {
            SET_PLAY_INHIVIT(fighter, Hash40::new("se_common_dizzy"), 10);
        }
    }
    else {
        frame(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("se_common_dizzy"));
        }
    }
}

//FuraFura
unsafe extern "C" fn sound_koopag_FuraFura(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FURAFURA_VOICE_FINAL) {
        wait_loop_sync_mot(fighter.lua_state_agent);
        wait(fighter.lua_state_agent, 200.0);
    }
    else {
        if is_excute(fighter) {
            PLAY_STATUS(fighter, Hash40::new("vc_koopag_furafura"));
        }
    }
    wait(fighter.lua_state_agent, 200.0);
}

pub fn install() {
    Agent::new("koopag")
	.sound_acmd("sound_specialnstart", sound_koopag_SpecialNStart, Low)
    .sound_acmd("sound_specialairnstart", sound_koopag_SpecialAirNStart, Low)
    .sound_acmd("sound_specialscatch", sound_koopag_SpecialSCatch, Low)
    .sound_acmd("sound_specialsaircatch", sound_koopag_SpecialSAirCatch, Low)
    .sound_acmd("sound_run", sound_koopag_Run, Low)
    .sound_acmd("sound_downboundd", sound_koopag_DownBoundD, Low)
    .sound_acmd("sound_downboundu", sound_koopag_DownBoundU, Low)
    .sound_acmd("sound_downattackd", sound_koopag_DownAttackD, Low)
    .sound_acmd("sound_downattacku", sound_koopag_DownAttackU, Low)
    .sound_acmd("sound_appealhil", sound_koopag_AppealHiL, Low)
    .sound_acmd("sound_appealhir", sound_koopag_AppealHiR, Low)
    .sound_acmd("sound_appeallwl", sound_koopag_AppealLwL, Low)
    .sound_acmd("sound_appeallwr", sound_koopag_AppealLwR, Low)
    .sound_acmd("sound_appealsl", sound_koopag_AppealSL, Low)
    .sound_acmd("sound_appealsr", sound_koopag_AppealSR, Low)
    .sound_acmd("sound_damagen1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagen2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagen3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagehi3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw1", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw2", sound_koopag_Damage, Low)
    .sound_acmd("sound_damagelw3", sound_koopag_Damage, Low)
    .sound_acmd("sound_damageflyhi", sound_koopag_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_koopag_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_koopag_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_koopag_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_koopag_DamageFlyRoll, Low)
    .sound_acmd("sound_shieldbreakfly", sound_koopag_ShieldBreakFly, Low)
    .sound_acmd("sound_furafurastartd", sound_koopag_FuraFuraStartD, Low)
    .sound_acmd("sound_furafurastartu", sound_koopag_FuraFuraStartU, Low)
    .sound_acmd("sound_furafura", sound_koopag_FuraFura, Low)
    .install();
}