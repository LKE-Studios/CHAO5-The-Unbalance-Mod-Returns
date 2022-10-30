use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//EntryL
    agent = "lucas", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_entryl(fighter: &mut L2CAgentBase) {
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
unsafe fn lucas_entryr(fighter: &mut L2CAgentBase) {
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
unsafe fn lucas_win1(fighter: &mut L2CAgentBase) {
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
unsafe fn lucas_win3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack13 SFX
    agent = "lucas", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_jab3sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
}

#[acmd_script(//AttackS4Charge
    agent = "lucas", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_sidesmashchargesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
    }
}

#[acmd_script(//AttackAirLwSFX
    agent = "lucas", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_dairsfx(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialLwStart
    agent = "lucas", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_downbsfx(fighter: &mut L2CAgentBase) {
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
unsafe fn lucas_downbairsfx(fighter: &mut L2CAgentBase) {
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
unsafe fn lucas_sidetauntlsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
    wait(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//AppealSR
    agent = "lucas", 
    script = "sound_appealsr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn lucas_sidetauntrsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
    wait(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        lucas_entryl,
        lucas_entryr,
        lucas_win1,
        lucas_win3,
        lucas_jab3sfx,
        lucas_sidesmashchargesfx,
        lucas_dairsfx,
        lucas_downbsfx,
        lucas_downbairsfx,
        lucas_sidetauntrsfx,
        lucas_sidetauntlsfx,
    );
}