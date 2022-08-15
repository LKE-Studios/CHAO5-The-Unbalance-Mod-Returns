use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "pitb", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glide1sfx(fighter: &mut L2CAgentBase) {
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
    }
}

/*#[acmd_script(//GlideWing
    agent = "pitb", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pitb_glide2sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pitb_special_h01"));
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

pub fn install() {
    smashline::install_acmd_scripts!(
        pitb_glide1sfx,
        //pitb_glide2sfx,
        pitb_glideattacksfx,
        pitb_glidelandingsfx,
        pitb_glideendsfx
    );
}