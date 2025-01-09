use crate::imports::BuildImports::*;

//AttackS4Charge
unsafe extern "C" fn sound_toonlink_AttackS4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackHi4Charge
unsafe extern "C" fn sound_toonlink_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//AttackLw4Charge
unsafe extern "C" fn sound_toonlink_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_toonlink_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_toonlink_rnd_futtobi01"), Hash40::new("seq_toonlink_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_toonlink_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_toonlink_rnd_futtobi01"), Hash40::new("seq_toonlink_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_toonlink_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_toonlink_rnd_futtobi01"), Hash40::new("seq_toonlink_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_toonlink_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_toonlink_rnd_futtobi01"), Hash40::new("seq_toonlink_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_toonlink_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_toonlink_rnd_futtobi01"), Hash40::new("seq_toonlink_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("toonlink")
    .sound_acmd("sound_attacks4charge", sound_toonlink_AttackS4Charge, Low)
    .sound_acmd("sound_attackhi4charge", sound_toonlink_AttackHi4Charge, Low)
    .sound_acmd("sound_attacklw4charge", sound_toonlink_AttackLw4Charge, Low)
    .sound_acmd("sound_damageflyhi", sound_toonlink_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_toonlink_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_toonlink_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_toonlink_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_toonlink_DamageFlyRoll, Low)
    .install();
}