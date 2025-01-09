use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_elight_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_elight_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_elight_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_elight_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_elight_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_elight_rnd_futtobi01"), Hash40::new("seq_elight_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("elight")
    .sound_acmd("sound_damageflyhi", sound_elight_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_elight_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_elight_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_elight_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_elight_DamageFlyRoll, Low)
    .install();
}