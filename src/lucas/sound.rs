use crate::imports::BuildImports::*;

#[acmd_script(//EntryL
    agent = "lucas", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_entryl(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EntryR
    agent = "lucas", 
    script = "sound_entryr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_entryr(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Win1
    agent = "lucas", 
    script = "sound_win1", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_win1"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win01"));
    }
}

#[acmd_script(//Win3
    agent = "lucas", 
    script = "sound_win3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_win3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack11
    agent = "lucas", 
    script = "sound_attack11", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attack11(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
            PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
        }
    } else {   
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        }
    }
}

#[acmd_script(//Attack12
    agent = "lucas", 
    script = "sound_attack12", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attack12(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
            PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
        }
    } else {   
        frame(fighter.lua_state_agent, 2.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        }
    }
}

#[acmd_script(//Attack13
    agent = "lucas", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attack13(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
            PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        }
    }
}

#[acmd_script(//AttackS4Charge
    agent = "lucas", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attacks4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
    }
}

#[acmd_script(//AttackLw4
    agent = "lucas", 
    script = "sound_attacklw4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attacklw4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
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
    } else { //Lucas
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
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
        }
        wait(fighter.lua_state_agent, 10.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_smash_l03"));
        }
    }
}

#[acmd_script(//AttackAirLw
    agent = "lucas", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
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
    } else { //Lucas
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
        }
        wait(fighter.lua_state_agent, 7.0);
        if is_excute(fighter) {
            PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
            PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l02"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
        }
    }
}

#[acmd_script(//SpecialLwStart
    agent = "lucas", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "lucas", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//AppealSL
    agent = "lucas", 
    script = "sound_appealsl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsl(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//AppealSR
    agent = "lucas", 
    script = "sound_appealsr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsr(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//WessDance
    agent = "lucas", 
    script = "sound_wessdance", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_wessdance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 1370.0);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_audience_cheer_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_lucas_entryl,
        sound_lucas_entryr,
        sound_lucas_win1,
        sound_lucas_win3,
        sound_lucas_attack11,
        sound_lucas_attack12,
        sound_lucas_attack13,
        sound_lucas_attacks4charge,
        sound_lucas_attacklw4,
        sound_lucas_attackairlw,
        sound_lucas_speciallwstart,
        sound_lucas_specialairlwstart,
        sound_lucas_appealsr,
        sound_lucas_appealsl,
        sound_lucas_wessdance
    );
}