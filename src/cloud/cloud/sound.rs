use crate::imports::BuildImports::*;

//SpecialLwLoop
unsafe extern "C" fn sound_cloud_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

//SpecialAirLwLoop
unsafe extern "C" fn sound_cloud_SpecialAirLwLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_cloud_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_cloud_rnd_futtobi01"), Hash40::new("seq_cloud_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_cloud_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_cloud_rnd_futtobi01"), Hash40::new("seq_cloud_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_cloud_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_cloud_rnd_futtobi01"), Hash40::new("seq_cloud_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_cloud_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_cloud_rnd_futtobi01"), Hash40::new("seq_cloud_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_cloud_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_cloud_rnd_futtobi01"), Hash40::new("seq_cloud_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("cloud")
    .sound_acmd("sound_speciallwloop", sound_cloud_SpecialLwLoop, Low)
    .sound_acmd("sound_specialairlwloop", sound_cloud_SpecialAirLwLoop, Low)
    .sound_acmd("sound_damageflyhi", sound_cloud_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_cloud_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_cloud_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_cloud_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_cloud_DamageFlyRoll, Low)
    .install();
}
