use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "trail", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_trail_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_trail_jump03"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_jump02"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_throw_shiny"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_special_h02"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_glide_loop"));
    }
}

#[acmd_script(//GlideAttack
    agent = "trail", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_trail_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_trail_attackair_b03"));
        PLAY_SE(fighter, Hash40::new("se_trail_attackhard_l02"));
        PLAY_SE(fighter, Hash40::new("se_trail_attackhard_s02"));
    }
}

#[acmd_script(//GlideLanding
    agent = "trail", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_trail_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_trail_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "trail", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_trail_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_trail_wing"));
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_trail_glidestart,
        sound_trail_glideattack,
        sound_trail_glideend,
        sound_trail_glidelanding
    );
}