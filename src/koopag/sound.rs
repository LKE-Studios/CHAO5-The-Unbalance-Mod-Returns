use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use smash::app::lua_bind::*;

#[acmd_script(//Run
    agent = "koopag", 
    script = "sound_run", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_koopag_001"));
    }
}

#[acmd_script(//SpecialNStart
    agent = "koopag", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_specialnstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_step_left_m"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

#[acmd_script(//SpecialAirNStart
    agent = "koopag", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_specialairnstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

#[acmd_script(//DownBoundD
    agent = "koopag", 
    script = "sound_downboundd", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_downboundd(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

#[acmd_script(//DownBoundU
    agent = "koopag", 
    script = "sound_downboundu", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_downboundu(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_s"));
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_m"));
        macros::STOP_SE(fighter, Hash40::new("se_common_blowaway_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_01"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_01"), 0.75);
    }
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_m_02"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_common_down_m_02"), 0.75);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_koopag_specialnstart,
        sound_koopag_specialairnstart,
        sound_koopag_run,
        sound_koopag_downboundd,
        sound_koopag_downboundu,
    );
}