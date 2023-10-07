use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "palutena", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_palutena_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_dash_start"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_jump02"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide")); //75
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide_loop")); //76
    }
}

/*#[acmd_script(//GlideWing
    agent = "palutena", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_palutena_glidewing(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_landing01_win01"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "palutena", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_palutena_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l03"));
    }
}

#[acmd_script(//GlideLanding
    agent = "palutena", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_palutena_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_palutena_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "palutena", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_palutena_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_escapeair"));
        PLAY_SE(fighter, Hash40::new("se_palutena_jump01"));
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_palutena_glidestart,
        //sound_palutena_glidewing,
        sound_palutena_glideattack,
        sound_palutena_glidelanding,
        sound_palutena_glideend
    );
}