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

//DamageFlyHi
unsafe extern "C" fn sound_pitb_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pitb_rnd_futtobi01"), Hash40::new("seq_pitb_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_pitb_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pitb_rnd_futtobi01"), Hash40::new("seq_pitb_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_pitb_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pitb_rnd_futtobi01"), Hash40::new("seq_pitb_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_pitb_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pitb_rnd_futtobi01"), Hash40::new("seq_pitb_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_pitb_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_pitb_rnd_futtobi01"), Hash40::new("seq_pitb_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("pitb")
    .sound_acmd("sound_jumpaerialf4", sound_pitb_JumpAerialF4, Low)
    .sound_acmd("sound_jumpaerialf5", sound_pitb_JumpAerialF5, Low)
    .sound_acmd("sound_jumpaerialf6", sound_pitb_JumpAerialF6, Low)
    .sound_acmd("sound_jumpaerialf4", sound_pitb_JumpAerialF7, Low)
    .sound_acmd("sound_glidestart", sound_pitb_GlideStart, Low)
    .sound_acmd("sound_glideattack", sound_pitb_GlideAttack, Low)
    .sound_acmd("sound_glidelanding", sound_pitb_GlideLanding, Low)
    .sound_acmd("sound_glideend", sound_pitb_GlideEnd, Low)
    .sound_acmd("sound_damageflyhi", sound_pitb_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_pitb_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_pitb_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_pitb_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_pitb_DamageFlyRoll, Low)
    .install();
}