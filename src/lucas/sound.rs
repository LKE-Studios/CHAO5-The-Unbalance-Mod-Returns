use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
//use crate::lucas::frame::*;

#[acmd_script(//EntryL
    agent = "lucas", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_entryl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_appear01"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 88.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_appear02"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_001"));
    }
}

#[acmd_script(//EntryR
    agent = "lucas", 
    script = "sound_entryr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_entryr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_appear01"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_landing01"));
    }
    frame(fighter.lua_state_agent, 88.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_appear02"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_001"));
    }
}

#[acmd_script(//Win1
    agent = "lucas", 
    script = "sound_win1", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_win1"));
    }
    frame(fighter.lua_state_agent, 65.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win01"));
    }
}

#[acmd_script(//Win3
    agent = "lucas", 
    script = "sound_win3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_win3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_win3"));
    }
    frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_008"));
    }
    frame(fighter.lua_state_agent, 123.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_win3_02"));
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
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
        }
    } else {   
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
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
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
        }
    } else {   
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
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
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
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
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
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
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
        }
        wait(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
        }
        wait(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
        }
        wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
        }
        wait(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_smash_l03"));
        }
    }
}


#[acmd_script(//AttackAirLwSFX
    agent = "lucas", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
        }
        wait(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
        }   
    } else { //Lucas
        frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
        }
        wait(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l02"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
        }
        wait(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
        }
    }
}

/*#[acmd_script(//SpecialS, SpecialAirS
    agent = "lucas", 
    scripts = ["sound_specials", "sound_specialairs"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_sidebsfx(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Claus
        CLAUS_PK_BEAM[ENTRY_ID] = true;
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            if CLAUS_PK_BEAM[ENTRY_ID] {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_002"));
                macros::PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
                SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_lucas_special_s03"), 0.7);
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_item_revengeshooter_shot"));
            }
        }
        frame(fighter.lua_state_agent, 20.0);
        if macros::is_excute(fighter) { 
            CLAUS_PK_BEAM[ENTRY_ID] = false;
        }
        else if CLAUS_PK_BEAM[ENTRY_ID] == false {
            frame(fighter.lua_state_agent, 5.0);
            if macros::is_excute(fighter) {
                macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_004"));
                macros::PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
            }
        }
    } else { //Lucas
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE_REMAIN(fighter, Hash40::new("vc_lucas_004"));
            macros::PLAY_SE(fighter, Hash40::new("se_lucas_special_s03"));
        }
    }
}*/

#[acmd_script(//SpecialLwStart
    agent = "lucas", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "lucas", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//AppealSL
    agent = "lucas", 
    script = "sound_appealsl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//AppealSR
    agent = "lucas", 
    script = "sound_appealsr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
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
        sound_lucas_appealsl
    );
}