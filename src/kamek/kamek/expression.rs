use crate::imports::BuildImports::*;

//SpecialHiStart
unsafe extern "C" fn expression_kamek_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn expression_kamek_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

//SpecialHiEnd
unsafe extern "C" fn expression_kamek_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn expression_kamek_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialLwHit
unsafe extern "C" fn expression_kamek_SpecialLwHit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirLwHit
unsafe extern "C" fn expression_kamek_SpecialAirLwHit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AppealSR
unsafe extern "C" fn expression_kamek_AppealSR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//AppealSL
unsafe extern "C" fn expression_kamek_AppealSL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    Agent::new("ness")
    .expression_acmd("expression_specialhistart_kamek", expression_kamek_SpecialHiStart, Low)
    .expression_acmd("expression_specialairhistart_kamek", expression_kamek_SpecialAirHiStart, Low)
    .expression_acmd("expression_specialhiend_kamek", expression_kamek_SpecialHiEnd, Low)
    .expression_acmd("expression_specialairhiend_kamek", expression_kamek_SpecialAirHiEnd, Low)
    .expression_acmd("expression_appealsr_kamek", expression_kamek_AppealSR, Low)
    .expression_acmd("expression_appealsl_kamek", expression_kamek_AppealSL, Low)
    .install();
}
