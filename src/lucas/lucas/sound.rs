use crate::imports::BuildImports::*;

//Attack13
unsafe extern "C" fn sound_lucas_Attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
}

//AttackS4Charge
unsafe extern "C" fn sound_lucas_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
    }
}

//AttackLw4
unsafe extern "C" fn sound_lucas_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l03"));
    }
}

//AttackAirLw
unsafe extern "C" fn sound_lucas_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l02"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
    }
}

//AppealSL
unsafe extern "C" fn sound_lucas_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

//AppealSR
unsafe extern "C" fn sound_lucas_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

//WessDance
unsafe extern "C" fn sound_lucas_WessDance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 1370.0);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_audience_cheer_l"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_lucas_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_lucas_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_lucas_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_lucas_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_lucas_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_lucas_rnd_futtobi01"), Hash40::new("seq_lucas_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("lucas")
    .sound_acmd("sound_attack13", sound_lucas_Attack13, Low)
    .sound_acmd("sound_attacks4charge", sound_lucas_AttackS4Charge, Low)
    .sound_acmd("sound_attacklw4", sound_lucas_AttackLw4, Low)
    .sound_acmd("sound_attackairlw", sound_lucas_AttackAirLw, Low)   
    .sound_acmd("sound_appealsr", sound_lucas_AppealSR, Low)    
    .sound_acmd("sound_appealsl", sound_lucas_AppealSL, Low)      
    .sound_acmd("sound_wessdance", sound_lucas_WessDance, Low)
    .sound_acmd("sound_damageflyhi", sound_lucas_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_lucas_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_lucas_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_lucas_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_lucas_DamageFlyRoll, Low)
    .install();
}