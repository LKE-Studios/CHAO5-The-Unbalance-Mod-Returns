use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "dolly",
    script =  "expression_attack11_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attack11(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack12
    agent = "dolly",
    script =  "expression_attack12_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attack12(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//Attack13
    agent = "dolly",
    script =  "expression_attack13_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attack13(fighter: &mut L2CAgentBase) {}

#[acmd_script(//AttackDash
    agent = "dolly",
    script =  "expression_attackdash_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_dolly_smash_l01"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_dolly_rnd_attackdash"));
    }
}

#[acmd_script(//AttackLw3 
    agent = "dolly", 
    script = "expression_attacklw3_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_attacklw3(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackLw4
    agent = "dolly",
    script =  "expression_attacklw4_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attacklw4(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirN
    agent = "dolly",
    script =  "expression_attackairn_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
}

#[acmd_script(//AttackAirF
    agent = "dolly",
    script =  "expression_attackairf_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackairf(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirB
    agent = "dolly",
    script =  "expression_attackairb_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//AttackAirHi
    agent = "dolly",
    script =  "expression_attackairhi_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackairhi(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AttackAirLw
    agent = "dolly",
    script =  "expression_attackairlw_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_attackairlw(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialSBStart
    agent = "dolly",
    script =  "expression_specialsbstart_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_specialsbstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialSBAttack
    agent = "dolly",
    script =  "expression_specialsbattack_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_specialsbattack(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSBAttackW
    agent = "dolly",
    script =  "expression_specialsbattackw_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_specialsbattackw(fighter: &mut L2CAgentBase) {}

#[acmd_script(//SpecialSFStart
    agent = "dolly",
    script =  "expression_specialsfstart_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_specialsfstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirSFStart
    agent = "dolly",
    script =  "expression_specialairsfstart_waluigi",
    category = ACMD_EXPRESSION, low_priority)]
unsafe fn expression_waluigi_specialairsfstart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealHiL 
    agent = "dolly", 
    script = "expression_appealhil_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_appealhil(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealHiR 
    agent = "dolly", 
    script = "expression_appealhir_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_appealhir(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealLwL 
    agent = "dolly", 
    script = "expression_appeallwl_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_appeallwl(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//AppealLwR 
    agent = "dolly", 
    script = "expression_appeallwr_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_appeallwr(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EntryR 
    agent = "dolly", 
    script = "expression_entryr_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_entryr(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//EntryL
    agent = "dolly", 
    script = "expression_entryl_waluigi", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_waluigi_entryl(fighter: &mut L2CAgentBase) {
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
    smashline::install_acmd_scripts!(
        expression_waluigi_attack11,
        expression_waluigi_attack12,
        expression_waluigi_attack13,
        expression_waluigi_attackdash,
        expression_waluigi_attacklw3,
        expression_waluigi_attacklw4,
        expression_waluigi_attackairn,
        expression_waluigi_attackairf,
        expression_waluigi_attackairb,
        expression_waluigi_attackairhi,
        expression_waluigi_attackairlw,
        expression_waluigi_specialsbstart,
        expression_waluigi_specialsbattackw,
        expression_waluigi_specialsbattack,
        expression_waluigi_specialsfstart,
        expression_waluigi_specialairsfstart,
        expression_waluigi_appealhil,
        expression_waluigi_appealhir,
        expression_waluigi_appeallwl,
        expression_waluigi_appeallwr,
        expression_waluigi_entryl,
        expression_waluigi_entryr,
    );
}