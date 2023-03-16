use smash::app::sv_animcmd::*;
use smash::app::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;
use smash::hash40;	
use crate::sonic::game::THROWHI_FRAME_LAND;

#[acmd_script(//Attack13 
    agent = "sonic", 
    script = "expression_attack13", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attack13(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
    } else {//Sonic
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
    }
}

#[acmd_script(//AttackDash
    agent = "sonic", 
    script = "expression_attackdash", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackdash(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 27.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    } else {//Sonic
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
    }
}

#[acmd_script(//AttackHi4
    agent = "sonic", 
    script = "expression_attackhi4", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackhi4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    } else {//Sonic
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_NONE);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 5);
        }
        frame(fighter.lua_state_agent, 21.0);
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        frame(fighter.lua_state_agent, 33.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        if macros::is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        }
        frame(fighter.lua_state_agent, 35.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        }
        frame(fighter.lua_state_agent, 42.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 50.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_lands"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

#[acmd_script(//AttackLw4Charge
    agent = "sonic", 
    script = "expression_attacklw4charge", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attacklw4charge(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, 1, 0.8, -1, Hash40::new("invalid"));
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
        }
        frame(fighter.lua_state_agent, 61.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    } else {//Sonic
        if macros::is_excute(fighter) {
            physics!(fighter, *MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, 1, 0.8, -1, Hash40::new("invalid"));
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 61.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

#[acmd_script(//AttackLw4
    agent = "sonic", 
    script = "expression_attacklw4", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attacklw4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        }
        frame(fighter.lua_state_agent, 6.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            if macros::is_excute(fighter) {
                slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
            }
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        }
        frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 8);
        }
        frame(fighter.lua_state_agent, 52.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        }
    } else {//Sonic  
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
        }
        frame(fighter.lua_state_agent, 6.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
            if macros::is_excute(fighter) {
                slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_L);
            }
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_TOP, 4);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 8);
        }
        frame(fighter.lua_state_agent, 52.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
        }
    }
}

#[acmd_script(//AttackAirN
    agent = "sonic", 
    script = "expression_attackairn", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackairn(fighter: &mut L2CAgentBase) {
	if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattack"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
        }
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 24.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 34.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 8, true, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 39.0);
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
        }
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
    }
}

#[acmd_script(//AttackAirF
    agent = "sonic", 
    script = "expression_attackairf", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackairf(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    } else { //Sonic
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 8, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackss"), 5);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

#[acmd_script(//AttackAirB
    agent = "sonic", 
    script = "expression_attackairb", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackairb(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

#[acmd_script(//AttackAirHi
    agent = "sonic", 
    script = "expression_attackairhi", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackairhi(fighter: &mut L2CAgentBase) { 
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 7.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

#[acmd_script(//AttackAirLw 
    agent = "sonic", 
    script = "expression_attackairlw", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    } else {//Sonic
        frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
    }
}

#[acmd_script(//ThrowLw 
    agent = "sonic", 
    script = "expression_throwlw", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_throwlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
        /*frame(fighter.lua_state_agent, 28.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 37.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 47.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 61.0);
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }*/
    } else {
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
        frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            lua_bind::VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        if macros::is_excute(fighter) {
            lua_bind::ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
        frame(fighter.lua_state_agent, 18.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
        }
        frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
            macros::RUMBLE_HIT(fighter, Hash40::new("rbkind_attacks"), 0);
        }
        frame(fighter.lua_state_agent, 34.0);
        if macros::is_excute(fighter) {
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
        frame(fighter.lua_state_agent, 41.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 10);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        frame(fighter.lua_state_agent, 51.0);
        if macros::is_excute(fighter) {
            lua_bind::VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        if macros::is_excute(fighter) {
            lua_bind::ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
        }
    }
}

#[acmd_script(//EntryL
    agent = "sonic", 
    script = "expression_entryl", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_entryl(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        }
        frame(fighter.lua_state_agent, 95.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 100.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
    } else {
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        }
        frame(fighter.lua_state_agent, 95.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 100.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
    }
}

#[acmd_script(//EntryR
    agent = "sonic", 
    script = "expression_entryr", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_sonic_entryr(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 {//Knuckles
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 40.0);
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        }
        frame(fighter.lua_state_agent, 95.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 100.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
    } else {
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), false);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_sphere") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_OFF);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_NORMAL);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            VisibilityModule::set_int64(fighter.module_accessor, hash40("body") as i64, hash40("body_normal") as i64);
            macros::HIT_NODE(fighter, Hash40::new("waist"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("s_stingd1"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("legl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("kneel"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footr"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("footl"), *HIT_STATUS_NORMAL);
            macros::HIT_NODE(fighter, Hash40::new("rot"), *HIT_STATUS_OFF);
        }
        if macros::is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, true, 0);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x1f20a9d549), true);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x24772eddef), true);
        }
        frame(fighter.lua_state_agent, 95.0);
        if macros::is_excute(fighter) {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_BACKSHIELD_INVISIBLE);
        }
        frame(fighter.lua_state_agent, 100.0);
        if macros::is_excute(fighter) {
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 10);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_sonic_attack13,
        expression_sonic_attackdash,
        expression_sonic_attackhi4,
        expression_sonic_attacklw4charge,
        expression_sonic_attacklw4,
        expression_sonic_attackairn,
        expression_sonic_attackairf,
        expression_sonic_attackairb,
        expression_sonic_attackairhi,
        expression_sonic_attackairlw,
        //expression_sonic_throwlw,
        expression_sonic_entryl,
        expression_sonic_entryr
    );
}