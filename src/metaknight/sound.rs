use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "metaknight", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_glide1sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_jump04"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
    }
}

#[acmd_script(//ThrowHiSFX
    agent = "metaknight", 
    script = "sound_throwhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_throwupsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    wait(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_metaknight_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_final_hit"));
    }
}   

#[acmd_script(//SpecialNStartSFX
    agent = "metaknight", 
    script = "sound_specialnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_neutralbstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialAirNStartSFX
    agent = "metaknight", 
    script = "sound_specialairnstart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_neutralbairstartsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_n01"));
    }
}

#[acmd_script(//SpecialHiSFX
    agent = "metaknight", 
    script = "sound_specialhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_upbsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_start"));
    }
    /*frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
    }*/
}    

#[acmd_script(//SpecialHiLoopSFX
    agent = "metaknight", 
    script = "sound_specialhiloop", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_upbloopsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_dash_start"));
    }
    /*frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("vc_metaknight_special_h01"));
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_special_h02"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
       macros::PLAY_STATUS(fighter, Hash40::new("se_metaknight_special_h03"));
    }*/
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        metaknight_glide1sfx,
        metaknight_throwupsfx,
        metaknight_neutralbstartsfx,
        metaknight_neutralbairstartsfx,
        metaknight_upbsfx,
        metaknight_upbloopsfx
    );
}