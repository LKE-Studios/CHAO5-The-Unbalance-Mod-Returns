use crate::imports::BuildImports::*;

//JumpAerialF3 
unsafe extern "C" fn sound_buddy_JumpAerialF3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"))
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_buddy_jump02_02"))
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_jump03_02"))
    }
}

//JumpAerialF4
unsafe extern "C" fn sound_buddy_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"))
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_buddy_jump02_02"))
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_jump03_02"))
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_buddy_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"))
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("vc_buddy_jump02_02"))
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_buddy_jump03_02"))
    }
}

//GlideStart
unsafe extern "C" fn sound_buddy_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_jump03_01"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_jump03_02"));
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_glide_loop"));
        PLAY_SE(fighter, Hash40::new("vc_buddy_special_h03_vc"));
    }
}

//GlideWing
unsafe extern "C" fn sound_buddy_GlideWing(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_buddy_wing"));
    }
}

//GlideAttack
unsafe extern "C" fn sound_buddy_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_buddy_rnd_attack11"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attackair_b01"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_attackair_b02"));
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_buddy_attackair_b03"));
        PLAY_SE(fighter, Hash40::new("se_buddy_attackair_b03"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_buddy_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_buddy_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_buddy_landing02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_buddy_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_buddy_rnd_futtobi01"), Hash40::new("seq_buddy_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_buddy_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_buddy_rnd_futtobi01"), Hash40::new("seq_buddy_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_buddy_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_buddy_rnd_futtobi01"), Hash40::new("seq_buddy_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_buddy_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_buddy_rnd_futtobi01"), Hash40::new("seq_buddy_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_buddy_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_buddy_rnd_futtobi01"), Hash40::new("seq_buddy_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("buddy")
    .sound_acmd("sound_jumpaerialf3", sound_buddy_JumpAerialF3, Low)
    .sound_acmd("sound_jumpaerialf4", sound_buddy_JumpAerialF4, Low)
    .sound_acmd("sound_jumpaerialf5", sound_buddy_JumpAerialF5, Low)
    .sound_acmd("sound_glidestart", sound_buddy_GlideStart, Low)
    .sound_acmd("sound_glidewing", sound_buddy_GlideWing, Low)
    .sound_acmd("sound_glideattack", sound_buddy_GlideAttack, Low)
    .sound_acmd("sound_glideend", sound_buddy_GlideEnd, Low)
    .sound_acmd("sound_glidelanding", sound_buddy_GlideLanding, Low)
    .sound_acmd("sound_damageflyhi", sound_buddy_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_buddy_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_buddy_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_buddy_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_buddy_DamageFlyRoll, Low)
    .install();
}