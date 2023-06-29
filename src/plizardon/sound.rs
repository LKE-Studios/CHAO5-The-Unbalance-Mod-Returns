use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, JumpAerialF6 
    agent = "plizardon", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5", "sound_jumpaerialf6"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_jumpaerialf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_special_h01_win02"));
        PLAY_SE_REMAIN(fighter, Hash40::new("se_plizardon_glide_loop"));        
    }
}

#[acmd_script(//GlideAttack
    agent = "plizardon", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_plizardon_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_tailswing"));
    }
}

#[acmd_script(//GlideLanding
    agent = "plizardon", 
    script = "sound_glidelanding", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_glidelanding(fighter: &mut L2CAgentBase) {
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
    agent = "plizardon", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_glideend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_plizardon_special_h01_win02"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_plizardon_wing"));
    }
}   

#[acmd_script(//Win2
    agent = "plizardon", 
    script = "sound_win2", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_plizardon_win2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_jump02"));
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_special_h01"));
    }
    frame(fighter.lua_state_agent, 49.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("vc_plizardon_win02"));
    }
    frame(fighter.lua_state_agent, 96.0);
    if is_excute(fighter) {
        PLAY_SE_NO_3D(fighter, Hash40::new("se_plizardon_landing02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_plizardon_jumpaerialf,
        sound_plizardon_glidestart,
        sound_plizardon_glideattack,
        sound_plizardon_glideend,
        sound_plizardon_glidelanding,
        sound_plizardon_win2
    );
}