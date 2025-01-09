use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_demon_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_demon_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_demon_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_demon_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_demon_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_demon_rnd_futtobi01"), Hash40::new("seq_demon_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("demon")
    .sound_acmd("sound_damageflyhi", sound_demon_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_demon_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_demon_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_demon_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_demon_DamageFlyRoll, Low)
    .install();
}