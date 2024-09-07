use crate::imports::BuildImports::*;

//JumpAerialF3
unsafe extern "C" fn sound_ridley_JumpAerialF3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_jump02"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
}

//JumpAerialF4
unsafe extern "C" fn sound_ridley_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_jump02"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_ridley_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_jump02"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_ridley_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_jump02"));
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_jump02_02"));
    }
}

//GlideStart
unsafe extern "C" fn sound_ridley_GlideStart(fighter: &mut L2CAgentBase) {
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

//GlideAttack
unsafe extern "C" fn sound_ridley_GlideAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_special_s01"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_ridley_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_ss"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_ridley_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ridley_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_ridley_wing"));
    }
}

pub fn install() {
    Agent::new("ridley")
    .sound_acmd("sound_jumpaerialf3", sound_ridley_JumpAerialF3, Low)
    .sound_acmd("sound_jumpaerialf4", sound_ridley_JumpAerialF4, Low)
    .sound_acmd("sound_jumpaerialf5", sound_ridley_JumpAerialF5, Low)
    .sound_acmd("sound_jumpaerialf6", sound_ridley_JumpAerialF6, Low)
    .sound_acmd("sound_glidestart", sound_ridley_GlideStart, Low)
    .sound_acmd("sound_glideattack", sound_ridley_GlideAttack, Low)
    .sound_acmd("sound_glidelanding", sound_ridley_GlideLanding, Low)
    .sound_acmd("sound_glideend", sound_ridley_GlideEnd, Low)
    .install();
}