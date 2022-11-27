use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "palutena", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn palutena_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_dash_start"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_jump02"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide")); //75
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide_loop")); //76
    }
}

/*#[acmd_script(//GlideWing
    agent = "palutena", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn palutena_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_landing01_win01"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "palutena", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn palutena_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_special_l03"));
    }
}

#[acmd_script(//GlideLanding
    agent = "palutena", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn palutena_glidelandingsfx(fighter: &mut L2CAgentBase) {
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
    agent = "palutena", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn palutena_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_escapeair"));
        macros::PLAY_SE(fighter, Hash40::new("se_palutena_jump01"));
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        palutena_glidestartsfx,
        //palutena_glidesfx,
        palutena_glideattacksfx,
        palutena_glidelandingsfx,
        palutena_glideendsfx
    );
}