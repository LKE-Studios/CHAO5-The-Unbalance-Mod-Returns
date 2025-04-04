use crate::imports::BuildImports::*;

//Attack11
unsafe extern "C" fn expression_waluigi_Attack11(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

//Attack12
unsafe extern "C" fn expression_waluigi_Attack12(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

//Attack13
unsafe extern "C" fn expression_waluigi_Attack13(fighter: &mut L2CAgentBase) {}

//AttackDash
unsafe extern "C" fn expression_waluigi_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_smash_l01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attackdash"));
    }
}

//AttackLw3 
unsafe extern "C" fn expression_waluigi_AttackLw3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

//AttackLw4
unsafe extern "C" fn expression_waluigi_AttackLw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

//AttackAirN
unsafe extern "C" fn expression_waluigi_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

//AttackAirF
unsafe extern "C" fn expression_waluigi_AttackAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

//AttackAirB
unsafe extern "C" fn expression_waluigi_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

//AttackAirHi
unsafe extern "C" fn expression_waluigi_AttackAirHi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

//AttackAirLw
unsafe extern "C" fn expression_waluigi_AttackAirLw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

//SpecialN
unsafe extern "C" fn expression_waluigi_SpecialN(fighter: &mut L2CAgentBase) {
    let dice_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        if dice_num == 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
        }
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3, true);
    }
    frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
	}
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
    }
}

//SpecialAirN
unsafe extern "C" fn expression_waluigi_SpecialAirN(fighter: &mut L2CAgentBase) { 
    let dice_num = WorkModule::get_int(fighter.module_accessor, *FIGHTER_WALUIGI_INSTANCE_WORK_ID_INT_DICEBLOCK_NUMBER);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        if dice_num == 0 {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), true);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
	if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_loopattack"), 15, true, *BATTLE_OBJECT_ID_INVALID as u32);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
    }
}

//SpecialSBStart
unsafe extern "C" fn expression_waluigi_SpecialSBStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialSBAttack
unsafe extern "C" fn expression_waluigi_SpecialSBAttack(fighter: &mut L2CAgentBase) {}

//SpecialSBAttackW
unsafe extern "C" fn expression_waluigi_SpecialSBAttackW(fighter: &mut L2CAgentBase) {}

//SpecialSFStart
unsafe extern "C" fn expression_waluigi_SpecialSFStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirSFStart
unsafe extern "C" fn expression_waluigi_SpecialAirSFStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    wait(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialLwJumpAir
unsafe extern "C" fn expression_waluigi_SpecialLwJumpAir(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_loopattack"), 15, true, *BATTLE_OBJECT_ID_INVALID as u32);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_1"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_2"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_3"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_4"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_5"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_6"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_7"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_8"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_9"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("num_dice_10"), false);
        ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("frame_dice"), false);
    }
}

//AppealHiL 
unsafe extern "C" fn expression_waluigi_AppealHiL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AppealHiR 
unsafe extern "C" fn expression_waluigi_AppealHiR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 53.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 3, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AppealLwL 
unsafe extern "C" fn expression_waluigi_AppealLwL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AppealLwR 
unsafe extern "C" fn expression_waluigi_AppealLwR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 90.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//EntryR 
unsafe extern "C" fn expression_waluigi_EntryR(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

//EntryL
unsafe extern "C" fn expression_waluigi_EntryL(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

pub fn install() {
    Agent::new("dolly")
    .expression_acmd("expression_attack11_waluigi", expression_waluigi_Attack11, Low)
    .expression_acmd("expression_attack12_waluigi", expression_waluigi_Attack12, Low)
    .expression_acmd("expression_attack13_waluigi", expression_waluigi_Attack13, Low)
    .expression_acmd("expression_attackdash_waluigi", expression_waluigi_AttackDash, Low)
    .expression_acmd("expression_attacklw3_waluigi", expression_waluigi_AttackLw3, Low)
    .expression_acmd("expression_attacklw4_waluigi", expression_waluigi_AttackLw4, Low)
    .expression_acmd("expression_attackairn_waluigi", expression_waluigi_AttackAirN, Low)
    .expression_acmd("expression_attackairf_waluigi", expression_waluigi_AttackAirF, Low)
    .expression_acmd("expression_attackairb_waluigi", expression_waluigi_AttackAirB, Low)
    .expression_acmd("expression_attackairhi_waluigi", expression_waluigi_AttackAirHi, Low)
    .expression_acmd("expression_attackairlw_waluigi", expression_waluigi_AttackAirLw, Low)
    .expression_acmd("expression_specialn_waluigi", expression_waluigi_SpecialN, Low)
    .expression_acmd("expression_specialairn_waluigi", expression_waluigi_SpecialAirN, Low)
    .expression_acmd("expression_specialsbstart_waluigi", expression_waluigi_SpecialSBStart, Low)
    .expression_acmd("expression_specialsbattackw_waluigi", expression_waluigi_SpecialSBAttackW, Low)
    .expression_acmd("expression_specialsbattack_waluigi", expression_waluigi_SpecialSBAttack, Low)
    .expression_acmd("expression_specialsfstart_waluigi", expression_waluigi_SpecialSFStart, Low)
    .expression_acmd("expression_specialairsfstart_waluigi", expression_waluigi_SpecialAirSFStart, Low)
    .expression_acmd("expression_speciallwjumpair", expression_waluigi_SpecialLwJumpAir, Low)
    .expression_acmd("expression_appealhil", expression_waluigi_AppealHiL, Low)
    .expression_acmd("expression_appealhir", expression_waluigi_AppealHiR, Low)
    .expression_acmd("expression_appeallwl", expression_waluigi_AppealLwL, Low)
    .expression_acmd("expression_appeallwr", expression_waluigi_AppealLwR, Low)
    .expression_acmd("expression_entryl_waluigi", expression_waluigi_EntryL, Low)
    .expression_acmd("expression_entryr_waluigi", expression_waluigi_EntryR, Low)
    .install();
}