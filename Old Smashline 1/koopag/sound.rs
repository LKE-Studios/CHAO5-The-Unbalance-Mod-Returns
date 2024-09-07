use crate::imports::BuildImports::*;

#[acmd_script(//Run
    agent = "koopag", 
    script = "sound_run", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_run(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopag_001"));
    }
}

#[acmd_script(//SpecialNStart
    agent = "koopag", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_specialnstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirNStart
    agent = "koopag", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_specialairnstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopag_special_n01"));
    }
    wait(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_koopag_special_n03"));
    }
}

#[acmd_script(//DownBoundD
    agent = "koopag", 
    script = "sound_downboundd", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_downboundd(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//DownBoundU
    agent = "koopag", 
    script = "sound_downboundu", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopag_downboundu(fighter: &mut L2CAgentBase) {
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

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_koopag_specialnstart,
        sound_koopag_specialairnstart,
        sound_koopag_run,
        sound_koopag_downboundd,
        sound_koopag_downboundu,
    );
}