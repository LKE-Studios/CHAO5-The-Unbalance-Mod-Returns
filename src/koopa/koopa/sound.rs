use crate::imports::BuildImports::*;

//AttackLw3
unsafe extern "C" fn sound_koopa_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
    }
}

//AttackAirLw 
unsafe extern "C" fn sound_koopa_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_koopa_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopa_rnd_futtobi01"), Hash40::new("seq_koopa_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_koopa_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopa_rnd_futtobi01"), Hash40::new("seq_koopa_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_koopa_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopa_rnd_futtobi01"), Hash40::new("seq_koopa_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_koopa_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopa_rnd_futtobi01"), Hash40::new("seq_koopa_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_koopa_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_koopa_rnd_futtobi01"), Hash40::new("seq_koopa_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("koopa")
    .sound_acmd("sound_attacklw3", sound_koopa_AttackLw3, Low)
    .sound_acmd("sound_attackairlw", sound_koopa_AttackAirLw, Low)
    .sound_acmd("sound_damageflyhi", sound_koopa_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_koopa_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_koopa_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_koopa_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_koopa_DamageFlyRoll, Low)
    .install();
}