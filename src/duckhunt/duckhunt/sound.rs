use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_duckhunt_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_duckhunt_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_duckhunt_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_duckhunt_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_duckhunt_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_duckhunt_rnd_futtobi01"), Hash40::new("seq_duckhunt_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("duckhunt")
    .sound_acmd("sound_damageflyhi", sound_duckhunt_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_duckhunt_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_duckhunt_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_duckhunt_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_duckhunt_DamageFlyRoll, Low)
    .install();
}