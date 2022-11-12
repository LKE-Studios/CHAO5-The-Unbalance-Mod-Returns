use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, 
    agent = "buddy", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_airjumpsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"))
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("vc_buddy_jump02_02"))
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_STATUS(fighter, Hash40::new("se_buddy_jump03_02"))
    }
}

#[acmd_script(//GlideStart
    agent = "buddy", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_jump03_02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_glide_loop"));
        macros::PLAY_SE(fighter, Hash40::new("vc_buddy_special_h03_vc"));
    }
}

/*#[acmd_script(//GlideWing
    agent = "buddy", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "buddy", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_buddy_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_buddy_attackair_b03"));
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_l02"));
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_attackhard_s02"));
    }
}

#[acmd_script(//GlideLanding
    agent = "buddy", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_glidelandingsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

/*#[acmd_script(//GlideEnd
    agent = "buddy", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn buddy_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_buddy_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
    }
}*/   

pub fn install() {
    smashline::install_acmd_scripts!(
        buddy_airjumpsfx,
        buddy_glidestartsfx,
        //buddy_glidesfx,
        buddy_glideattacksfx,
        //buddy_glideendsfx,
        buddy_glidelandingsfx
    );
}