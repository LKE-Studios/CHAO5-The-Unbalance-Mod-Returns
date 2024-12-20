use crate::imports::BuildImports::*;

//Turn
unsafe extern "C" fn expression_maskedman_Turn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//SpecialNDash
unsafe extern "C" fn expression_maskedman_SpecialNDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//SpecialAirNDash
unsafe extern "C" fn expression_maskedman_SpecialAirNDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
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

//SpecialLwStart
unsafe extern "C" fn expression_maskedman_SpecialLwStart(fighter: &mut L2CAgentBase) {
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_MASKEDMAN_STATUS_SPECIAL_LW_WORK_INT_TIME);
    if is_excute(fighter) {
        if int_time == 4 {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}

//SpecialAirLwStart
unsafe extern "C" fn expression_maskedman_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    let int_time = WorkModule::get_int(fighter.module_accessor, FIGHTER_MASKEDMAN_STATUS_SPECIAL_LW_WORK_INT_TIME);
    if is_excute(fighter) {
        if int_time == 4 {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}


pub fn install() {
    Agent::new("lucas")
    .expression_acmd("expression_turn_maskedman", expression_maskedman_Turn, Low)
    .expression_acmd("expression_specialndash_maskedman", expression_maskedman_SpecialNDash, Low)
    .expression_acmd("expression_specialairndash_maskedman", expression_maskedman_SpecialAirNDash, Low)
    .expression_acmd("expression_specialhistart_maskedman", expression_maskedman_SpecialHiStart, Low)
    .expression_acmd("expression_specialhihold_maskedman", expression_maskedman_SpecialHiHold, Low)
    .expression_acmd("expression_specialairhistart_maskedman", expression_maskedman_SpecialAirHiStart, Low)
    .expression_acmd("expression_speciallwstart_maskedman", expression_maskedman_SpecialLwStart, Low)
    .expression_acmd("expression_specialairlwstart_maskedman", expression_maskedman_SpecialAirLwStart, Low)
    .install();
}