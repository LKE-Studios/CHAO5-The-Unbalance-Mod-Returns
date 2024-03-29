use crate::imports::BuildImports::*;

//JumpAerialF4
unsafe extern "C" fn sound_pitb_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_pitb_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_pitb_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

//JumpAerialF7
unsafe extern "C" fn sound_pitb_JumpAerialF7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
}

//GlideStart
unsafe extern "C" fn sound_pitb_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump02"));
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_glide_start")); //Index 52
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_pitb_glide_loop")); //Index 53
    }
}

//GlideAttack
unsafe extern "C" fn sound_pitb_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_swing_m"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_pitb_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_pitb_landing02"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_pitb_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_pitb_bowsplit"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_pitb_jump01"));
    }
}   

pub fn install() {
    Agent::new("pitb")
    .sound_acmd("sound_jumpaerialf4", sound_pitb_JumpAerialF4)
    .sound_acmd("sound_jumpaerialf5", sound_pitb_JumpAerialF5)
    .sound_acmd("sound_jumpaerialf6", sound_pitb_JumpAerialF6)
    .sound_acmd("sound_jumpaerialf4", sound_pitb_JumpAerialF7)
    .sound_acmd("sound_glidestart", sound_pitb_GlideStart)
    .sound_acmd("sound_glideattack", sound_pitb_GlideAttack)
    .sound_acmd("sound_glidelanding", sound_pitb_GlideLanding)
    .sound_acmd("sound_glideend", sound_pitb_GlideEnd)
    .install();
}