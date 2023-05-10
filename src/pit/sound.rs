use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7
    agent = "pit", 
    scripts = ["sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6", "sound_jumpaerialf7"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pit_jumpaerialf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
}

#[acmd_script(//GlideStart
    agent = "pit", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pit_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_glide_start"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        //macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_bowsplit"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_glide_loop"));
    }
}

#[acmd_script(//GlideAttack
    agent = "pit", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pit_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_swing_m"));
    }
}

#[acmd_script(//GlideLanding
    agent = "pit", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pit_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_LANDING_SE(fighter, Hash40::new("se_pit_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "pit", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_pit_glideend(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_pit_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump01"));
    }
}  

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_pit_jumpaerialf,
        sound_pit_glidestart,
        sound_pit_glideattack,
        sound_pit_glidelanding,
        sound_pit_glideend
    );
}