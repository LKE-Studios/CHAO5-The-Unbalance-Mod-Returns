use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//EntryL
    agent = "claus", 
    script = "sound_entryl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_entryl(fighter: &mut L2CAgentBase) {
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
    agent = "claus", 
    script = "sound_entryr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_entryr(fighter: &mut L2CAgentBase) {
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
    agent = "claus", 
    script = "sound_win1", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_win1(fighter: &mut L2CAgentBase) {
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
    agent = "claus", 
    script = "sound_win3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_win3(fighter: &mut L2CAgentBase) {
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
    agent = "claus", 
    script = "sound_attack11", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
    }
} 

#[acmd_script(//Attack12
    agent = "claus", 
    script = "sound_attack12", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_s"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_attackhard_s01"));
    }
}

#[acmd_script(//Attack13
    agent = "claus", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
} 

#[acmd_script(//AttackS4Charge
    agent = "claus", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attacks4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
    }
}

#[acmd_script(//AttackLw4
    agent = "claus", 
    script = "sound_attacklw4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attacklw4(fighter: &mut L2CAgentBase) {
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
}

#[acmd_script(//AttackAirLw
    agent = "claus", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_attackairlw(fighter: &mut L2CAgentBase) {
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
}

#[acmd_script(//SpecialLwStart
    agent = "claus", 
    script = "sound_speciallwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_speciallwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//SpecialAirLwStart
    agent = "claus", 
    script = "sound_specialairlwstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_specialairlwstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_002"));
    }
}

#[acmd_script(//AppealSL
    agent = "claus", 
    script = "sound_appealsl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_appealsl(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//AppealSR
    agent = "claus", 
    script = "sound_appealsr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_appealsr(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//WessDance
    agent = "claus", 
    script = "sound_wessdance", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_claus_wessdance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_claus_entryl,
        sound_claus_entryr,
        sound_claus_win1,
        sound_claus_win3,
        sound_claus_attack11,
        sound_claus_attack12,
        sound_claus_attack13,
        sound_claus_attacks4charge,
        sound_claus_attacklw4,
        sound_claus_attackairlw,
        sound_claus_speciallwstart,
        sound_claus_specialairlwstart,
        sound_claus_appealsr,
        sound_claus_appealsl,
        sound_claus_wessdance
    );
}