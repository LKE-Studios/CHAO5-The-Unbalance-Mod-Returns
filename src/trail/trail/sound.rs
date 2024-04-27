use crate::imports::BuildImports::*;

//GlideStart
unsafe extern "C" fn sound_trail_GlideStart(fighter: &mut L2CAgentBase) {
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

//GlideAttack
unsafe extern "C" fn sound_trail_GlideAttack(fighter: &mut L2CAgentBase) {
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

//GlideLanding
unsafe extern "C" fn sound_trail_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_trail_landing02"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_trail_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_trail_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_trail_wing"));
    }
}   

pub fn install() {
    Agent::new("trail")
    .sound_acmd("sound_glidestart", sound_trail_GlideStart, Low)
    .sound_acmd("sound_glideattack", sound_trail_GlideAttack, Low)
    .sound_acmd("sound_glideend", sound_trail_GlideEnd, Low)
    .sound_acmd("sound_glidelanding", sound_trail_GlideLanding, Low)
    .install();
}
