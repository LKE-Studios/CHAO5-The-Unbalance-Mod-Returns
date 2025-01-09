use crate::imports::BuildImports::*;

//AttackDash 
unsafe extern "C" fn sound_younglink_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l02"));
        PLAY_SE(fighter, Hash40::new("se_younglink_escape"))
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_landing01"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_younglink_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_younglink_attack04"));
        PLAY_SE(fighter, Hash40::new("se_common_smashswing_03"));
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_ll"));
    }
    wait(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_ll"));
    }
}

//AttackAirF
unsafe extern "C" fn sound_younglink_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_younglink_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_younglink_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_younglink_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_younglink_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_younglink_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_younglink_rnd_futtobi01"), Hash40::new("seq_younglink_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("younglink")
    .sound_acmd("sound_attackdash", sound_younglink_AttackDash, Low)
    .sound_acmd("sound_attackhi4", sound_younglink_AttackHi4, Low)
    .sound_acmd("sound_attackairf", sound_younglink_AttackAirF, Low)   
    .sound_acmd("sound_damageflyhi", sound_younglink_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_younglink_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_younglink_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_younglink_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_younglink_DamageFlyRoll, Low) 
    .install();
}