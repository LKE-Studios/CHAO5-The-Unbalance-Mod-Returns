use crate::imports::BuildImports::*;

//SpecialZCharge
unsafe extern "C" fn expression_pfushigisou_SpecialZCharge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialZShoot
unsafe extern "C" fn expression_pfushigisou_SpecialZShoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//SpecialZEnd
unsafe extern "C" fn expression_pfushigisou_SpecialZEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//SpecialAirZCharge
unsafe extern "C" fn expression_pfushigisou_SpecialAirZCharge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirZShoot
unsafe extern "C" fn expression_pfushigisou_SpecialAirZShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

//SpecialAirZEnd
unsafe extern "C" fn expression_pfushigisou_SpecialAirZEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .expression_acmd("expression_specialzcharge", expression_pfushigisou_SpecialZCharge)
    .expression_acmd("expression_specialzend", expression_pfushigisou_SpecialZEnd)
    .expression_acmd("expression_specialzshoot", expression_pfushigisou_SpecialZShoot)
    .expression_acmd("expression_specialairzcharge", expression_pfushigisou_SpecialAirZCharge)
    .expression_acmd("expression_specialairzend", expression_pfushigisou_SpecialAirZEnd)
    .expression_acmd("expression_specialairzshoot", expression_pfushigisou_SpecialAirZShoot)
    .install();
}