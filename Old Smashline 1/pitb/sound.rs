use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pitb", 
    scripts = ["sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6", "sound_jumpaerialf7"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pitb_jumpaerialf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

#[acmd_script(//GlideStart
    agent = "pitb", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pitb_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_glide_start")); //Index 52
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_glide_loop")); //Index 53
    }
}

#[acmd_script(//GlideAttack
    agent = "pitb", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pitb_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
    }
}

#[acmd_script(//GlideLanding
    agent = "pitb", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pitb_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_pitb_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "pitb", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pitb_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_pitb_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump01"));
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_pitb_jumpaerialf,
        sound_pitb_glidestart,
        sound_pitb_glideattack,
        sound_pitb_glidelanding,
        sound_pitb_glideend
    );
}