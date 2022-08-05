use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn plizardon_glide1sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
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
        plizardon_glide1sfx,
        plizardon_win2sfx
    );
}