use crate::imports::BuildImports::*;

//AttackDash
unsafe extern "C" fn expression_younglink_AttackDash(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_walk_hv"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//AttackHi4
unsafe extern "C" fn expression_younglink_AttackHi4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_L, 3);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

//AttackAirF
unsafe extern "C" fn expression_younglink_AttackAirF(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword"), AttackDirectionAxis(*ATTACK_DIRECTION_Z_MINUS), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y));
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
}

pub fn install() {
    Agent::new("younglink")
    .expression_acmd("expression_attackdash", expression_younglink_AttackDash, Low)
    .expression_acmd("expression_attackhi4", expression_younglink_AttackHi4, Low)
    .expression_acmd("expression_attackairf", expression_younglink_AttackAirF, Low)    
    .install();
}