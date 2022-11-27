use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, JumpAerialF6 
    agent = "plizardon", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_airjumpsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_loop"));        
    }
}

/*#[acmd_script(//GlideWing
    agent = "plizardon", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pickel_special_h02"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "plizardon", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glideattacksfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_plizardon_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_tailswing"));
    }
}

#[acmd_script(//GlideLanding
    agent = "plizardon", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glidelandingsfx(fighter: &mut L2CAgentBase) {
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
    agent = "plizardon", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}   

#[acmd_script(//Win2
    agent = "plizardon", 
    script = "sound_win2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_win2sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_jump02"));
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_special_h01"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_plizardon_win02"));
    }
    frame(fighter.lua_state_agent, 96.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        plizardon_airjumpsfx,
        plizardon_glidestartsfx,
        //plizardon_glidesfx,
        plizardon_glideattacksfx,
        plizardon_glideendsfx,
        plizardon_glidelandingsfx,
        plizardon_win2sfx
    );
}