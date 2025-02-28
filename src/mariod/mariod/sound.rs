use crate::imports::BuildImports::*;

//AttackS4Hi
unsafe extern "C" fn sound_mariod_AttackS4Hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

//AttackS4
unsafe extern "C" fn sound_mariod_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

//AttackS4Lw
unsafe extern "C" fn sound_mariod_AttackS4Lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
        PLAY_SE(fighter, Hash40::new("vc_mariod_attack05"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_mariod_smash_s01"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_mariod_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mariod_rnd_futtobi01"), Hash40::new("seq_mariod_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_mariod_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mariod_rnd_futtobi01"), Hash40::new("seq_mariod_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_mariod_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mariod_rnd_futtobi01"), Hash40::new("seq_mariod_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_mariod_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mariod_rnd_futtobi01"), Hash40::new("seq_mariod_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_mariod_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_mariod_rnd_futtobi01"), Hash40::new("seq_mariod_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("mariod")
    .sound_acmd("sound_attacks4hi", sound_mariod_AttackS4Hi, Low)
    .sound_acmd("sound_attacks4", sound_mariod_AttackS4, Low)
    .sound_acmd("sound_attacks4lw", sound_mariod_AttackS4Lw, Low)
    .sound_acmd("sound_damageflyhi", sound_mariod_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_mariod_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_mariod_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_mariod_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_mariod_DamageFlyRoll, Low)
    .install();
}