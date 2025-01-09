use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_zelda_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_zelda_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_zelda_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_zelda_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_zelda_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_zelda_rnd_futtobi01"), Hash40::new("seq_zelda_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("zelda")
    .sound_acmd("sound_damageflyhi", sound_zelda_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_zelda_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_zelda_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_zelda_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_zelda_DamageFlyRoll, Low)
    .install();
}