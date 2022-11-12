use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pitb", 
    scripts = ["sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6", "sound_jumpaerialf7"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_airjumpsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

#[acmd_script(//GlideStart
    agent = "pitb", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_special_h01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_bowsplit"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_landing01_win01"));
    }
}

/*#[acmd_script(//GlideWing
    agent = "pitb", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_landing01_win01"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "pitb", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
    }
}

#[acmd_script(//GlideLanding
    agent = "pitb", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

#[acmd_script(//GlideEnd
    agent = "pitb", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_pitb_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_jump01"));
    }
}   

#[acmd_script(//Win1
    agent = "pitb", 
    scripts = ["sound_win1", "sound_win1_default", "sound_win1_us_en"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_pitb_win01"));
    }
    frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_landing02"));
    }
    frame(fighter.lua_state_agent, 64.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_pitb_rise"));
    }
    frame(fighter.lua_state_agent, 88.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
    }
    frame(fighter.lua_state_agent, 92.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_win1_win01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pitb_airjumpsfx,
        pitb_glidestartsfx,
        //pitb_glidesfx,
        pitb_glideattacksfx,
        pitb_glidelandingsfx,
        pitb_glideendsfx,
        pitb_win1
    );
}