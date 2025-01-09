use crate::imports::BuildImports::*;

//SpecialLwLaugh
unsafe extern "C" fn sound_diddy_SpecialLwLaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

//SpecialAirLwLaugh
unsafe extern "C" fn sound_diddy_SpecialAirLwLaugh(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_diddy_furafura"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("vc_diddy_furafura"), 1.1);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_diddy_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_diddy_rnd_futtobi01"), Hash40::new("seq_diddy_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_diddy_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_diddy_rnd_futtobi01"), Hash40::new("seq_diddy_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_diddy_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_diddy_rnd_futtobi01"), Hash40::new("seq_diddy_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_diddy_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_diddy_rnd_futtobi01"), Hash40::new("seq_diddy_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_diddy_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_diddy_rnd_futtobi01"), Hash40::new("seq_diddy_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("diddy")
    .sound_acmd("sound_speciallwlaugh", sound_diddy_SpecialLwLaugh, Low)
    .sound_acmd("sound_specialairlwlaugh", sound_diddy_SpecialAirLwLaugh, Low)
    .sound_acmd("sound_damageflyhi", sound_diddy_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_diddy_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_diddy_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_diddy_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_diddy_DamageFlyRoll, Low)
    .install();
}