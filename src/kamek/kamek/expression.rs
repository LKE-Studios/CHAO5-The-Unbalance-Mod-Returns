use crate::imports::BuildImports::*;

//SpecialLwHold
unsafe extern "C" fn expression_ninten_SpecialLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter,0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
}

//SpecialAirLwHold
unsafe extern "C" fn expression_ninten_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter,0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
}

//SpecialLwHit
unsafe extern "C" fn expression_ninten_SpecialLwHit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirLwHit
unsafe extern "C" fn expression_ninten_SpecialAirLwHit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AppealSR
unsafe extern "C" fn expression_ninten_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//AppealSL
unsafe extern "C" fn expression_ninten_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ness")
    .expression_acmd("expression_speciallwhold_ninten", expression_ninten_SpecialLwHold, Low)
    .expression_acmd("expression_specialairlwhold_ninten", expression_ninten_SpecialAirLwHold, Low)
    .expression_acmd("expression_speciallwhit_ninten", expression_ninten_SpecialLwHit, Low)
    .expression_acmd("expression_specialairlwhit_ninten", expression_ninten_SpecialAirLwHit, Low)
    .expression_acmd("expression_appealsr_ninten", expression_ninten_AppealSR, Low)
    .expression_acmd("expression_appealsl_ninten", expression_ninten_AppealSL, Low)
    .install();
}