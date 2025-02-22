use crate::imports::BuildImports::*;

//EscapeB
unsafe extern "C" fn expression_sans_EscapeB(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EscapeF
unsafe extern "C" fn expression_sans_EscapeF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -8, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//EscapeN
unsafe extern "C" fn expression_sans_EscapeN(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 20.0);
}

//EscapeAir
unsafe extern "C" fn expression_sans_EscapeAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("rot"), 0, -8, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 2.0, 2.0, 2.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 23.0);
}

//AttackS4Charge
unsafe extern "C" fn expression_sans_AttackS4Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AttackS4
unsafe extern "C" fn expression_sans_AttackS4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(fighter.lua_state_agent, 11.0);
    execute(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 65.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

//AttackHi4Charge
unsafe extern "C" fn expression_sans_AttackHi4Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AttackHi4
unsafe extern "C" fn expression_sans_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 13.0);
    execute(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beamm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

//AttackLw4Charge
unsafe extern "C" fn expression_sans_AttackLw4Charge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 61.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }   
}

//AttackLw4
unsafe extern "C" fn expression_sans_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 4);
    }
    frame(fighter.lua_state_agent, 14.0);
    execute(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 55.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

//SpecialS
unsafe extern "C" fn expression_sans_SpecialS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirS
unsafe extern "C" fn expression_sans_SpecialAirS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//EntryL
unsafe extern "C" fn expression_sans_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

//EntryR
unsafe extern "C" fn expression_sans_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 45.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), true);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        VisibilityModule::set_model_visible(fighter.module_accessor, true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("sans_heartwhite"), false);
    }
    frame(fighter.lua_state_agent, 100.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 8);
    }
}

pub fn install() {
    Agent::new("palutena")
    .expression_acmd("expression_escapeb_sans", expression_sans_EscapeB, Low)
    .expression_acmd("expression_escapef_sans", expression_sans_EscapeF, Low)
    .expression_acmd("expression_escapen_sans", expression_sans_EscapeN, Low)
    .expression_acmd("expression_escapeair_sans", expression_sans_EscapeAir, Low)
    .expression_acmd("expression_attacks4charge_sans", expression_sans_AttackS4Charge, Low)
    .expression_acmd("expression_attacks4_sans", expression_sans_AttackS4, Low)
    .expression_acmd("expression_attackhi4charge_sans", expression_sans_AttackHi4Charge, Low)
    .expression_acmd("expression_attackhi4_sans", expression_sans_AttackHi4, Low)
    .expression_acmd("expression_attacklw4charge_sans", expression_sans_AttackLw4Charge, Low)
    .expression_acmd("expression_attacklw4_sans", expression_sans_AttackLw4, Low)
    .expression_acmd("expression_specials_sans", expression_sans_SpecialS, Low)
    .expression_acmd("expression_specialairs_sans", expression_sans_SpecialAirS, Low)
    .expression_acmd("expression_entryl_sans", expression_sans_EntryL, Low)
    .expression_acmd("expression_entryr_sans", expression_sans_EntryR, Low)
    .install();
}