use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "pit", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_glidestartsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_special_h01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_bowsplit"));
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_landing02_win01"));
    }
}

/*#[acmd_script(//GlideWing
    agent = "pit", 
    script = "sound_glidewing", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_glidesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_pit_landing02_win01"));
    }
}*/

#[acmd_script(//GlideAttack
    agent = "pit", 
    script = "sound_glideattack", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_glideattacksfx(fighter: &mut L2CAgentBase) {
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
unsafe fn pit_glidelandingsfx(fighter: &mut L2CAgentBase) {
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
    agent = "pit", 
    script = "sound_glideend", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_glideendsfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_pit_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump01"));
    }
}  

#[acmd_script(//Win1
    agent = "pit", 
    scripts = ["sound_win1", "sound_win1_default", "sound_win1_us_en"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_win1(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_landing02"));
    }
    frame(fighter.lua_state_agent, 66.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_swing_s"));
    }
    frame(fighter.lua_state_agent, 77.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_swing_s"));
    }
    frame(fighter.lua_state_agent, 105.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_NO_3D(fighter, Hash40::new("vc_pit_win01"));
    }
    frame(fighter.lua_state_agent, 128.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_swing_m"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pit_glidestartsfx,
        //pit_glidesfx,
        pit_glideattacksfx,
        pit_glidelandingsfx,
        pit_glideendsfx,
        pit_win1
    );
}