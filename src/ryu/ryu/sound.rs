use crate::imports::BuildImports::*;

//Kamehameha_Start
unsafe extern "C" fn sound_ryu_Kamehameha_Start(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_ryu_special_n01"));
    }
}

//Kamehameha_Charge
unsafe extern "C" fn sound_ryu_Kamehameha_Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_item_genesis_shot01"));
    }
}

//Kamehameha_Fire
unsafe extern "C" fn sound_ryu_Kamehameha_Fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_item_genesis_shot02"));
    }
    frame(fighter.lua_state_agent, 70.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_item_genesis_shot02"));
    }
}

//DamageFlyHi
unsafe extern "C" fn sound_ryu_DamageFlyHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ryu_rnd_futtobi01"), Hash40::new("seq_ryu_rnd_futtobi02"));
    }
}

//DamageFlyLw
unsafe extern "C" fn sound_ryu_DamageFlyLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ryu_rnd_futtobi01"), Hash40::new("seq_ryu_rnd_futtobi02"));
    }
}

//DamageFlyN
unsafe extern "C" fn sound_ryu_DamageFlyN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ryu_rnd_futtobi01"), Hash40::new("seq_ryu_rnd_futtobi02"));
    }
}

//DamageFlyTop
unsafe extern "C" fn sound_ryu_DamageFlyTop(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ryu_rnd_futtobi01"), Hash40::new("seq_ryu_rnd_futtobi02"));
    }
}

//DamageFlyRoll
unsafe extern "C" fn sound_ryu_DamageFlyRoll(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_FLY_VOICE(fighter, Hash40::new("seq_ryu_rnd_futtobi01"), Hash40::new("seq_ryu_rnd_futtobi02"));
    }
}

pub fn install() {
    Agent::new("ryu")
    .sound_acmd("sound_kamehameha_start", sound_ryu_Kamehameha_Start, Low)
    .sound_acmd("sound_kamehameha_charge", sound_ryu_Kamehameha_Charge, Low)
    .sound_acmd("sound_kamehameha_fire", sound_ryu_Kamehameha_Fire, Low)
    .sound_acmd("sound_damageflyhi", sound_ryu_DamageFlyHi, Low)
    .sound_acmd("sound_damageflylw", sound_ryu_DamageFlyLw, Low)
    .sound_acmd("sound_damageflyn", sound_ryu_DamageFlyN, Low)
    .sound_acmd("sound_damageflytop", sound_ryu_DamageFlyTop, Low)
    .sound_acmd("sound_damageflyroll", sound_ryu_DamageFlyRoll, Low)
    .install();
}