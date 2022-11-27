use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "trail", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn trail_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_trail_jump03"));
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_jump02"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_throw_shiny"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_special_h02"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_trail_glide_loop"));
    }
}

/*#[acmd_script(//GlideWing
    agent = "trail", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn trail_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_trail_wing"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "trail", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn trail_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_trail_rnd_attack11"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_trail_attackair_b03"));
        macros::PLAY_SE(fighter, Hash40::new("se_trail_attackhard_l02"));
        macros::PLAY_SE(fighter, Hash40::new("se_trail_attackhard_s02"));
    }
}

#[acmd_script(//GlideLanding
    agent = "trail", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn trail_glidelandingsfx(fighter: &mut L2CAgentBase) {
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
    agent = "trail", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn trail_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_trail_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_trail_wing"));
    }
}*/   

pub fn install() {
    smashline::install_acmd_scripts!(
        trail_glidestartsfx,
        //trail_glidesfx,
        trail_glideattacksfx,
        //trail_glideendsfx,
        trail_glidelandingsfx
    );
}