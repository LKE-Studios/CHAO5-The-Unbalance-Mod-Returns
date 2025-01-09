use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_captain_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_captain_rnd_futtobi01"), Hash40::new("seq_captain_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_captain_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_captain_rnd_futtobi01"), Hash40::new("seq_captain_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_captain_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_captain_rnd_futtobi01"), Hash40::new("seq_captain_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_captain_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_captain_rnd_futtobi01"), Hash40::new("seq_captain_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_captain_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_captain_rnd_futtobi01"), Hash40::new("seq_captain_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("captain")
    .sound_acmd("sound_damageflyhi", sound_captain_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_captain_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_captain_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_captain_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_captain_DamageFlyRoll, Low)
    .install();
}