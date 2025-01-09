use crate::imports::BuildImports::*;

//SpecialHiH
unsafe extern "C" fn sound_kirby_SpecialHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match sv_math::rand(hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

//SpecialAirHiH
unsafe extern "C" fn sound_kirby_SpecialAirHiH(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h02"));
        match sv_math::rand(hash40("fighter"), 2) {
            0 => PLAY_SE(fighter, Hash40::new("vc_kirby_002")),
            1 => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
            _ => PLAY_SE(fighter, Hash40::new("vc_kirby_attack07")),
        };
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_kirby_special_h04"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_kirby_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_kirby_rnd_futtobi01"), Hash40::new("seq_kirby_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_kirby_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_kirby_rnd_futtobi01"), Hash40::new("seq_kirby_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_kirby_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_kirby_rnd_futtobi01"), Hash40::new("seq_kirby_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_kirby_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_kirby_rnd_futtobi01"), Hash40::new("seq_kirby_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_kirby_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_kirby_rnd_futtobi01"), Hash40::new("seq_kirby_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("kirby")
    .sound_acmd("sound_specialhih", sound_kirby_SpecialHiH, Low)
    .sound_acmd("sound_specialairhih", sound_kirby_SpecialAirHiH, Low)
    .sound_acmd("sound_damageflyhi", sound_kirby_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_kirby_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_kirby_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_kirby_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_kirby_DamageFlyRoll, Low)
    .install();
}