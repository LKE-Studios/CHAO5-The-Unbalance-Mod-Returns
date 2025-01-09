use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_koopajr_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopajr_rnd_futtobi01"), Hash40::new("seq_koopajr_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_koopajr_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopajr_rnd_futtobi01"), Hash40::new("seq_koopajr_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_koopajr_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopajr_rnd_futtobi01"), Hash40::new("seq_koopajr_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_koopajr_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopajr_rnd_futtobi01"), Hash40::new("seq_koopajr_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_koopajr_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopajr_rnd_futtobi01"), Hash40::new("seq_koopajr_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("koopajr")
    .sound_acmd("sound_damageflyhi", sound_koopajr_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_koopajr_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_koopajr_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_koopajr_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_koopajr_DamageFlyRoll, Low)
    .install();
}