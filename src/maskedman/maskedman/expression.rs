use crate::imports::BuildImports::*;

//Turn
unsafe extern "C" fn expression_maskedman_Turn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//SpecialHiStart
unsafe extern "C" fn expression_maskedman_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn expression_maskedman_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

//SpecialHiHold
unsafe extern "C" fn expression_maskedman_SpecialHiHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_80_special_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_80_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("lucas")
    .expression_acmd("expression_turn_maskedman", expression_maskedman_Turn, Low)
    .expression_acmd("expression_specialhistart_maskedman", expression_maskedman_SpecialHiStart, Low)
    .expression_acmd("expression_specialhihold_maskedman", expression_maskedman_SpecialHiHold, Low)
    .expression_acmd("expression_specialairhistart_maskedman", expression_maskedman_SpecialAirHiStart, Low)
    .install();
}