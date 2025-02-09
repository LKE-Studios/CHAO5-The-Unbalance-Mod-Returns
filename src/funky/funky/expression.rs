use crate::imports::BuildImports::*;

//AttackS4
unsafe extern "C" fn expression_funky_AttackS4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    execute(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_loopattack"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_loopattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AttackLw4Charge
unsafe extern "C" fn expression_funky_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        physics!(fighter,*MA_MSC_CMD_PHYSICS_START_CHARGE, 0.8, 0.8, -1, 0.8, 0.8, -1, Hash40::new("toer2"));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AttackLw4
unsafe extern "C" fn expression_funky_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 9.0 + 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 9.0 + 5.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 9.0 + 16.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

//SpecialN
unsafe extern "C" fn expression_funky_SpecialN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//SpecialNMax
unsafe extern "C" fn expression_funky_SpecialNMax(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//SpecialAirN
unsafe extern "C" fn expression_funky_SpecialAirN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

//SpecialAirNMax
unsafe extern "C" fn expression_funky_SpecialAirNMax(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitll"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}
//SpecialHiEnd
unsafe extern "C" fn expression_funky_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landl_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

//SpecialAirHi
unsafe extern "C" fn expression_funky_SpecialAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_stg_power_up"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirHiLaunch
unsafe extern "C" fn expression_funky_SpecialAirHiLaunch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialHi_C2
unsafe extern "C" fn expression_funky_SpecialHi_C2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 38.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 47.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 81.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

pub fn install() {
    Agent::new("donkey")
    .expression_acmd("expression_attacks4_funky", expression_funky_AttackS4, Low)
    .expression_acmd("expression_attacklw4charge_funky", expression_funky_AttackLw4Charge, Low)
    .expression_acmd("expression_attacklw4_funky", expression_funky_AttackLw4, Low)
    .expression_acmd("expression_specialn_funky", expression_funky_SpecialN, Low)
    .expression_acmd("expression_specialnmax_funky", expression_funky_SpecialNMax, Low)
    .expression_acmd("expression_specialairn_funky", expression_funky_SpecialAirN, Low)
    .expression_acmd("expression_specialairnmax_funky", expression_funky_SpecialAirNMax, Low)
    .expression_acmd("expression_specialhiend_funky", expression_funky_SpecialHiEnd, Low)
    .expression_acmd("expression_specialairhi_funky", expression_funky_SpecialAirHi, Low)
    .expression_acmd("expression_specialairhilaunch_funky", expression_funky_SpecialAirHiLaunch, Low)
    .expression_acmd("expression_specialhic2_funky", expression_funky_SpecialHi_C2, Low)
    .install();
}