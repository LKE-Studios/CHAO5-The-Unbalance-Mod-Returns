use crate::imports::BuildImports::*;

//SpecialNStart
unsafe extern "C" fn expression_krystal_SpecialNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_none") as i64);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//SpecialAirNStart
unsafe extern "C" fn expression_krystal_SpecialAirNStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_none") as i64);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_PITB_GENERATE_ARTICLE_BOW, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

//SpecialNFireS
unsafe extern "C" fn expression_krystal_SpecialNFireS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

//SpecialAirNFireS
unsafe extern "C" fn expression_krystal_SpecialAirNFireS(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_N_CHARGE_FLAG_CHARGE_MAX) {
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
    else {
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

//SpecialSStart
unsafe extern "C" fn expression_krystal_SpecialSStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_rush"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), true);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 15);
    }
    frame(fighter.lua_state_agent, 75.0);
    if is_excute(fighter) {
        VisibilityModule::set_default_int64(fighter.module_accessor, hash40("weapon") as i64);
    }
}

//SpecialAirSStart
unsafe extern "C" fn expression_krystal_SpecialAirSStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_rush"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 50.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), true);
    }
}

//SpecialSEnd
unsafe extern "C" fn expression_krystal_SpecialSEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 6, true);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        VisibilityModule::set_default_int64(fighter.module_accessor, hash40("weapon") as i64);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 60.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), true);
    }
}

//SpecialAirSEnd
unsafe extern "C" fn expression_krystal_SpecialAirSEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), true);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), true);
    }
}

//SpecialLwStartL
unsafe extern "C" fn expression_krystal_SpecialLwStartL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
    }
}

//SpecialLwStartR
unsafe extern "C" fn expression_krystal_SpecialLwStartR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
    }
}

//SpecialAirLwStartL
unsafe extern "C" fn expression_krystal_SpecialAirLwStartL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
    }
}

//SpecialAirLwStartR
unsafe extern "C" fn expression_krystal_SpecialAirLwStartR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
    }
}

//SpecialLwHold
unsafe extern "C" fn expression_krystal_SpecialLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirLwHold
unsafe extern "C" fn expression_krystal_SpecialAirLwHold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_bow_r") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("weapon") as i64, hash40("weapon_arm_r") as i64);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_awaken"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//EntryL
unsafe extern "C" fn expression_krystal_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//EntryR
unsafe extern "C" fn expression_krystal_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, false);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 1);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

//Win2
unsafe extern "C" fn expression_krystal_Win2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
}

//Win2Wait
unsafe extern "C" fn expression_krystal_Win2Wait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponlm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_arm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_weaponrm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_yumirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_wpnnagirm"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("pit_final"), false);
    }
}

pub fn install() {
    Agent::new("pitb")
    .expression_acmd("expression_specialnstart_krystal", expression_krystal_SpecialNStart, Low)  
    .expression_acmd("expression_specialairnstart_krystal", expression_krystal_SpecialAirNStart, Low)  
    .expression_acmd("expression_specialsstart_krystal", expression_krystal_SpecialSStart, Low)  
    .expression_acmd("expression_specialairsstart_krystal", expression_krystal_SpecialAirSStart, Low)  
    .expression_acmd("expression_specialsend_krystal", expression_krystal_SpecialSEnd, Low)  
    .expression_acmd("expression_specialairsend_krystal", expression_krystal_SpecialAirSEnd, Low) 
    .expression_acmd("expression_speciallwstartl_krystal", expression_krystal_SpecialLwStartL, Low)
    .expression_acmd("expression_speciallwstartr_krystal", expression_krystal_SpecialLwStartR, Low) 
    .expression_acmd("expression_specialairlwstartl_krystal", expression_krystal_SpecialAirLwStartL, Low)
    .expression_acmd("expression_specialairlwstartr_krystal", expression_krystal_SpecialAirLwStartR, Low) 
    .expression_acmd("expression_speciallwhold_krystal", expression_krystal_SpecialLwHold, Low)
    .expression_acmd("expression_specialairlwhold_krystal", expression_krystal_SpecialAirLwHold, Low)
    .expression_acmd("expression_entryl_krystal", expression_krystal_EntryL, Low)
    .expression_acmd("expression_entryr_krystal", expression_krystal_EntryR, Low)
    .expression_acmd("expression_win2_krystal", expression_krystal_Win2, Low)
    .expression_acmd("expression_win2wait_krystal", expression_krystal_Win2Wait, Low)
    .install();
}