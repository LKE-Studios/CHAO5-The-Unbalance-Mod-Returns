use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_richter_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_richter_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_richter_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_richter_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_richter_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_richter_rnd_futtobi01"), Hash40::new("seq_richter_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("richter")
    .sound_acmd("sound_damageflyhi", sound_richter_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_richter_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_richter_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_richter_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_richter_DamageFlyRoll, Low)
    .install();
}