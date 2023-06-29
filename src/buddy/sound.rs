use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, 
    agent = "buddy", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_buddy_jumpaerial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"))
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_buddy_jump02_02"))
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_jump03_02"))
    }
}

#[acmd_script(//GlideStart
    agent = "buddy", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_buddy_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_jump03_02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_glide_loop"));
        PLAY_SE(fighter, Hash40::new("vc_buddy_special_h03_vc"));
    }
}

#[acmd_script(//GlideAttack
    agent = "buddy", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_buddy_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_buddy_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_buddy_attackair_b03"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_l02"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_s02"));
    }
}

#[acmd_script(//GlideLanding
    agent = "buddy", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_buddy_glidelanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_buddy_landing02"));
    }
}

#[acmd_script(//GlideEnd
    agent = "buddy", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_buddy_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
    }
}   

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_buddy_jumpaerial,
        sound_buddy_glidestart,
        sound_buddy_glideattack,
        sound_buddy_glideend,
        sound_buddy_glidelanding
    );
}