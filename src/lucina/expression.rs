use crate::imports::BuildImports::*;

#[acmd_script(//Attack13
    agent = "lucina", 
    script = "expression_attack13", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attack13(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_pierces"), 0);
    }
}

#[acmd_script(//AttackS3Hi
    agent = "lucina", 
    script = "expression_attacks3hi", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attacks3hi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercem"), 0);
    }
}

#[acmd_script(//AttackS3
    agent = "lucina", 
    script = "expression_attacks3", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attacks3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercem"), 0);
    }
}

#[acmd_script(//AttackS3Lw
    agent = "lucina", 
    script = "expression_attacks3lw", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attacks3lw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercem"), 0);
    }
}

#[acmd_script(//AttackS4
    agent = "lucina", 
    script = "expression_attacks4", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attacks4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
}

#[acmd_script(//AttackHi4
    agent = "lucina", 
    script = "expression_attackhi4", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attackhi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        AREA_WIND_2ND_arg10(fighter, 0, 1, 80, 300, 0.8, 0, 12, 24, 24, 80);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 0);
    }
}

#[acmd_script(//AttackLw4
    agent = "lucina", 
    script = "expression_attacklw4", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attacklw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 2, true);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 14, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 40.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 16);
    }
}

#[acmd_script(//AttackAirB
    agent = "lucina", 
    script = "expression_attackairb", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_attackairb(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_piercem"), 0);
    }
}

#[acmd_script(//SpecialLwHit2
    agent = "lucina", 
    script = "expression_speciallwhit2", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_speciallwhit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
}

#[acmd_script(//SpecialAirLwHit2
    agent = "lucina", 
    script = "expression_specialairlwhit2", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_lucina_specialairlwhit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_counter"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_lucina_attack13,
        expression_lucina_attacks3hi,
        expression_lucina_attacks3,
        expression_lucina_attacks3lw,
        expression_lucina_attacks4,
        expression_lucina_attackhi4,
        expression_lucina_attacklw4,
        expression_lucina_attackairb,
        expression_lucina_speciallwhit2,
        expression_lucina_specialairlwhit2
    );
}