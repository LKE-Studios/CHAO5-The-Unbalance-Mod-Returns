use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_roy_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_roy_rnd_futtobi01"), Hash40::new("seq_roy_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_roy_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_roy_rnd_futtobi01"), Hash40::new("seq_roy_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_roy_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_roy_rnd_futtobi01"), Hash40::new("seq_roy_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_roy_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_roy_rnd_futtobi01"), Hash40::new("seq_roy_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_roy_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_roy_rnd_futtobi01"), Hash40::new("seq_roy_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("roy")
    .sound_acmd("sound_damageflyhi", sound_roy_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_roy_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_roy_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_roy_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_roy_DamageFlyRoll, Low)
    .install();
}