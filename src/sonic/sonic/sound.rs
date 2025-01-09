use crate::imports::BuildImports::*;

//Attack12
unsafe extern "C" fn sound_sonic_Attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_m"));
    }
}

//Attack13
unsafe extern "C" fn sound_sonic_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_l"));
    }
}

//AttackDash
unsafe extern "C" fn sound_sonic_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_sonic_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_swing_ll"));
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_right_m"));
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_step_left_m"));
    }
}

//AttackHi4
unsafe extern "C" fn sound_sonic_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new_raw(0x13589f8893));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack06"));
        PLAY_SE(fighter, Hash40::new("se_sonic_smash_h01"));
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_sonic_smash_h01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_sonic_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_sonic_attack07"));
        PLAY_STATUS(fighter, Hash40::new("se_sonic_smash_l01"));
    }
    wait(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_sonic_landing02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_sonic_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_sonic_rnd_futtobi01"), Hash40::new("seq_sonic_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_sonic_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_sonic_rnd_futtobi01"), Hash40::new("seq_sonic_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_sonic_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_sonic_rnd_futtobi01"), Hash40::new("seq_sonic_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_sonic_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_sonic_rnd_futtobi01"), Hash40::new("seq_sonic_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_sonic_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_sonic_rnd_futtobi01"), Hash40::new("seq_sonic_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("sonic")
    .sound_acmd("sound_attack12", sound_sonic_Attack12, Low)
    .sound_acmd("sound_attack13", sound_sonic_Attack13, Low)
    .sound_acmd("sound_attackdash", sound_sonic_AttackDash, Low)
    .sound_acmd("sound_attackhi4", sound_sonic_AttackHi4, Low)
    .sound_acmd("sound_attacklw4", sound_sonic_AttackLw4, Low)
    .sound_acmd("sound_damageflyhi", sound_sonic_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_sonic_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_sonic_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_sonic_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_sonic_DamageFlyRoll, Low)
    .install();
}