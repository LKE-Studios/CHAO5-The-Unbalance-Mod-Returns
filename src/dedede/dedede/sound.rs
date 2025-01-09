use crate::imports::BuildImports::*;

//JumpAerialF3
unsafe extern "C" fn sound_dedede_JumpAerialF3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF4
unsafe extern "C" fn sound_dedede_JumpAerialF4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF5
unsafe extern "C" fn sound_dedede_JumpAerialF5(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF6
unsafe extern "C" fn sound_dedede_JumpAerialF6(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF7
unsafe extern "C" fn sound_dedede_JumpAerialF7(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF8
unsafe extern "C" fn sound_dedede_JumpAerialF8(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//JumpAerialF9
unsafe extern "C" fn sound_dedede_JumpAerialF9(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_dedede_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_dedede_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_dedede_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_dedede_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_dedede_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_dedede_rnd_futtobi01"), Hash40::new("seq_dedede_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("dedede")
    .sound_acmd("sound_jumpaerialf3", sound_dedede_JumpAerialF3, Low)
    .sound_acmd("sound_jumpaerialf4", sound_dedede_JumpAerialF4, Low)
    .sound_acmd("sound_jumpaerialf5", sound_dedede_JumpAerialF5, Low)
    .sound_acmd("sound_jumpaerialf6", sound_dedede_JumpAerialF6, Low)
    .sound_acmd("sound_jumpaerialf7", sound_dedede_JumpAerialF7, Low)
    .sound_acmd("sound_jumpaerialf8", sound_dedede_JumpAerialF8, Low)
    .sound_acmd("sound_jumpaerialf9", sound_dedede_JumpAerialF9, Low)
    .sound_acmd("sound_damageflyhi", sound_dedede_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_dedede_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_dedede_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_dedede_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_dedede_DamageFlyRoll, Low)
    .install();
}