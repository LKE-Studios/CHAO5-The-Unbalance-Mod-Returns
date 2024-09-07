use crate::imports::BuildImports::*;

#[acmd_script(//WalkSlow 
    agent = "mewtwo", 
    script = "expression_walkslow_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_walkslow(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//WalkMiddle 
    agent = "mewtwo", 
    script = "expression_walkmiddle_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_walkmiddle(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//WalkFast 
    agent = "mewtwo", 
    script = "expression_walkfast_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_walkfast(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 6);
        }
        frame(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//Run 
    agent = "mewtwo", 
    script = "expression_run_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_run(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            AREA_WIND_2ND_arg10(fighter, 0, 2, 0, 300, 1, 13, 7, 32, 14, 30);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        }
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script(//RunBrakeR 
    agent = "mewtwo", 
    script = "expression_runbraker_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_runbraker(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script(//RunBrakeL 
    agent = "mewtwo", 
    script = "expression_runbrakel_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_runbrakel(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script(//TurnRunBrake 
    agent = "mewtwo", 
    script = "expression_turnrunbrake_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_turnrunbrake(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script(//Attack11 
    agent = "mewtwo", 
    script = "expression_attack11_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attack11(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}

#[acmd_script(//AttackDash
    agent = "mewtwo", 
    script = "expression_attackdash_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackdash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AttackS3Hi 
    agent = "mewtwo", 
    script = "expression_attacks3hi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks3hi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 6);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

#[acmd_script(//AttackS3 
    agent = "mewtwo", 
    script = "expression_attacks3_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 6);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

#[acmd_script(//AttackS3Lw 
    agent = "mewtwo", 
    script = "expression_attacks3lw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks3lw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 3);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 6);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 2);
    }
}

#[acmd_script(//AttackHi3 
    agent = "mewtwo", 
    script = "expression_attackhi3_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackhi3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 5, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script(//AttackLw3 
    agent = "mewtwo", 
    script = "expression_attacklw3_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacklw3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_TOP);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//AttackS4Hi 
    agent = "mewtwo", 
    script = "expression_attacks4hi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks4hi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 13.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script(//AttackS4 
    agent = "mewtwo", 
    script = "expression_attacks4_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 13.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script(//AttackS4Lw 
    agent = "mewtwo", 
    script = "expression_attacks4lw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacks4lw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 13.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        }
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

#[acmd_script(//AttackHi4 
    agent = "mewtwo", 
    script = "expression_attackhi4_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackhi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
        HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
    }
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 58.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AttackLw4 
    agent = "mewtwo", 
    script = "expression_attacklw4_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attacklw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
    }
    frame(fighter.lua_state_agent, 7.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 8);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script(//AttackAirN 
    agent = "mewtwo", 
    script = "expression_attackairn_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AttackAirF 
    agent = "mewtwo", 
    script = "expression_attackairf_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//AttackAirB 
    agent = "mewtwo", 
    script = "expression_attackairb_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//AttackAirHi 
    agent = "mewtwo", 
    script = "expression_attackairhi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//AttackAirLw 
    agent = "mewtwo", 
    script = "expression_attackairlw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
}

#[acmd_script(//SpecialNStart 
    agent = "mewtwo", 
    script = "expression_specialnstart_silver", 
    category = ACMD_EXPRESSION, low_priority )]
unsafe fn expression_silver_specialnstart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
}

#[acmd_script(//SpecialAirNStart 
    agent = "sonic", 
    script = "expression_specialairnstart_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairnstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
}

#[acmd_script(//SpecialNShoot 
    agent = "mewtwo", 
    script = "expression_specialnshoot_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialnshoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialAirNShoot 
    agent = "mewtwo", 
    script = "expression_specialairnshoot_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairnshoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 52.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialSStart 
    agent = "mewtwo", 
    script = "expression_specialsstart_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialsstart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
}

#[acmd_script(//SpecialAirSStart 
    agent = "mewtwo", 
    script = "expression_specialairsstart_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairsstart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
    }
}

#[acmd_script(//SpecialS 
    agent = "mewtwo", 
    script = "expression_specials_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specials(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_24_psyco"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialAirS 
    agent = "mewtwo", 
    script = "expression_specialairs_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairs(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 62.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_24_psyco"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialHiStart 
    agent = "mewtwo", 
    script = "expression_specialhistart_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialAirHiStart 
    agent = "mewtwo", 
    script = "expression_specialairhistart_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairhistart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialHi 
    agent = "mewtwo", 
    script = "expression_specialhi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

#[acmd_script(//SpecialAirHi 
    agent = "mewtwo", 
    script = "expression_specialairhi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beamss"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
    }
}

#[acmd_script(//SpecialLw 
    agent = "mewtwo", 
    script = "expression_speciallw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_speciallw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(fighter, 0, 0.5, 220, 3, 0.2, -10, 5, 20, 10, 80);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

#[acmd_script(//SpecialAirLw 
    agent = "mewtwo", 
    script = "expression_specialairlw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_specialairlw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(fighter, 0, 0.5, 220, 3, 0.2, -10, 5, 20, 10, 80);
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

#[acmd_script(//ThrowF 
    agent = "mewtwo", 
    script = "expression_throwf_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_throwf(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

#[acmd_script(//ThrowB 
    agent = "mewtwo", 
    script = "expression_throwb_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_throwb(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
    }
}

#[acmd_script(//ThrowHi 
    agent = "mewtwo", 
    script = "expression_throwhi_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_throwhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("top"), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

#[acmd_script(//ThrowLw 
    agent = "mewtwo", 
    script = "expression_throwlw_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_throwlw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FT_ATTACK_ABS_CAMERA_QUAKE(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, *CAMERA_QUAKE_KIND_NONE);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 10);
    }
    frame(fighter.lua_state_agent, 54.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 59.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 16);
    }
}

#[acmd_script(//AppealSL
    agent = "mewtwo", 
    script = "expression_appealsl_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_appealsl(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AppealSR
    agent = "mewtwo", 
    script = "expression_appealsr_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_appealsr(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 117.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AppealHiR 
    agent = "mewtwo", 
    script = "expression_appealhir_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_appealhir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 165.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AppealHiL
    agent = "mewtwo", 
    script = "expression_appealhil_silver", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_silver_appealhil(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 31.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 42.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 165.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_silver_walkslow,
        expression_silver_walkmiddle,
        expression_silver_walkfast,
        expression_silver_run,
        expression_silver_runbraker,
        expression_silver_runbrakel,
        expression_silver_turnrunbrake,
        expression_silver_attack11,
        expression_silver_attackdash,
        expression_silver_attacks3hi,
        expression_silver_attacks3,
        expression_silver_attacks3lw,
        expression_silver_attackhi3,
        expression_silver_attacklw3,
        expression_silver_attacks4hi,
        expression_silver_attacks4,
        expression_silver_attacks4lw,
        expression_silver_attackhi4,
        expression_silver_attacklw4,
        expression_silver_attackairn,
        expression_silver_attackairf,
        expression_silver_attackairb,
        expression_silver_attackairhi,
        expression_silver_attackairlw,
        expression_silver_throwf,
        expression_silver_throwb,
        expression_silver_throwhi,
        expression_silver_throwlw,
        expression_silver_specialnstart,
        expression_silver_specialairnstart,
        expression_silver_specialnshoot,
        expression_silver_specialairnshoot,
        expression_silver_specialsstart,
        expression_silver_specialairsstart,
        expression_silver_specials,
        expression_silver_specialairs,
        expression_silver_specialhistart,
        expression_silver_specialairhistart,
        expression_silver_specialhi,
        expression_silver_specialairhi,
        expression_silver_speciallw,
        expression_silver_specialairlw,
        expression_silver_appealsl,
        expression_silver_appealsr,
        expression_silver_appealhil,
        expression_silver_appealhir
    );
}

