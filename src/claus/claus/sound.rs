use crate::imports::BuildImports::*;

//EntryL
unsafe extern "C" fn sound_claus_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appear01"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appear02"));
        PLAY_SE(fighter, Hash40::new("vc_lucas_001"));
    }
}

//EntryR
unsafe extern "C" fn sound_claus_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appear01"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 88.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_appear02"));
        PLAY_SE(fighter, Hash40::new("vc_lucas_001"));
    }
}

//Win1
unsafe extern "C" fn sound_claus_Win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_win1"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win01"));
    }
}

//Win3
unsafe extern "C" fn sound_claus_Win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 43.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_win3"));
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_008"));
    }
    frame(fighter.lua_state_agent, 123.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_win3_02"));
    }        
}

//Attack11
unsafe extern "C" fn sound_claus_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
    }
} 

//Attack12
unsafe extern "C" fn sound_claus_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
    }
}

//Attack13
unsafe extern "C" fn sound_claus_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
} 

//AttackLw4
unsafe extern "C" fn sound_claus_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_claus_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
    }   
}

//SpecialLwStart
unsafe extern "C" fn sound_claus_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

//SpecialAirLwStart
unsafe extern "C" fn sound_claus_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

//AppealSL
unsafe extern "C" fn sound_claus_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

//AppealSR
unsafe extern "C" fn sound_claus_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

//WessDance
unsafe extern "C" fn sound_claus_WessDance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
}

pub fn install() {
    Agent::new("lucas")
    .sound_acmd("sound_entryl_claus", sound_claus_EntryL)
    .sound_acmd("sound_entryr_claus", sound_claus_EntryR)
    .sound_acmd("sound_win1_claus", sound_claus_Win1)
    .sound_acmd("sound_win3_claus", sound_claus_Win3)
    .sound_acmd("sound_attack11_claus", sound_claus_Attack11)
    .sound_acmd("sound_attack12_claus", sound_claus_Attack12)
    .sound_acmd("sound_attack13_claus", sound_claus_Attack13)
    .sound_acmd("sound_attacklw4_claus", sound_claus_AttackLw4)
    .sound_acmd("sound_attackairlw_claus", sound_claus_AttackAirLw)
    .sound_acmd("sound_speciallwstart_claus", sound_claus_SpecialLwStart)    
    .sound_acmd("sound_specialairlwstart_claus", sound_claus_SpecialAirLwStart)    
    .sound_acmd("sound_appealsr_claus", sound_claus_AppealSR)    
    .sound_acmd("sound_appealsl_claus", sound_claus_AppealSL)      
    .sound_acmd("sound_wessdance_claus", sound_claus_WessDance)
    .install();
}