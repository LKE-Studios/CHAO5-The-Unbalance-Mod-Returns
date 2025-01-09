use crate::imports::BuildImports::*;

//DamageFlyHi
unsafe extern "C" fn sound_mewtwo_DamageFlyHi(fighter: &mut L2CAgentBase) {
    let rand_fly_voice = sv_math::rand(hash40("mewtwo"), 3);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if rand_fly_voice == 0 {
            PLAY_SE(fighter, Hash40::new("vc_mewtwo_damagefly02"));
        }
        else {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
        }
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_mewtwo_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_mewtwo_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_mewtwo_DamageFlyTop(fighter: &mut L2CAgentBase) {
    let rand_fly_voice = sv_math::rand(hash40("mewtwo"), 3);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if rand_fly_voice == 0 {
            PLAY_SE(fighter, Hash40::new("vc_mewtwo_damagefly02"));
        }
        else {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
        }
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_mewtwo_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    let rand_fly_voice = sv_math::rand(hash40("mewtwo"), 3);
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        if rand_fly_voice == 0 {
            PLAY_SE(fighter, Hash40::new("vc_mewtwo_damagefly02"));
        }
        else {
            PLAY_FLY_VOICE(fighter, Hash40::new("seq_mewtwo_rnd_futtobi01"), Hash40::new("seq_mewtwo_rnd_futtobi02"));
        }
    }
}

pub fn install() {
    Agent::new("mewtwo")
    .sound_acmd("sound_damageflyhi", sound_mewtwo_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_mewtwo_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_mewtwo_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_mewtwo_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_mewtwo_DamageFlyRoll, Low)
    .install();
}