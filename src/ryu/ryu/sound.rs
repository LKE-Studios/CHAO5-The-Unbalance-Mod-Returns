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

pub fn install() {
    Agent::new("ryu")
    .sound_acmd("sound_kamehameha_start", sound_ryu_Kamehameha_Start)
    .sound_acmd("sound_kamehameha_charge", sound_ryu_Kamehameha_Charge)
    .sound_acmd("sound_kamehameha_fire", sound_ryu_Kamehameha_Fire)
    .install();
}