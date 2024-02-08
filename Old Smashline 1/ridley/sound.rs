use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, JumpAerialF6 
    agent = "ridley", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ridley_jumpaerialf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_jump02"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
}

#[acmd_script(//GlideStart
    agent = "ridley", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ridley_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_glide_start"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_special_h01"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_ridley_glide_loop"));
    }
}

#[acmd_script(//GlideAttack
    agent = "ridley", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ridley_glideattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_s01"));
    }
}

#[acmd_script(//GlideLanding
    agent = "ridley", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ridley_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

#[acmd_script(//GlideEnd
    agent = "ridley", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_ridley_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_wing"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_ridley_jumpaerialf,
        sound_ridley_glidestart,
        sound_ridley_glideattack,
        sound_ridley_glideend,
        sound_ridley_glidelanding
    );
}