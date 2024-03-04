use crate::imports::BuildImports::*;

//AscendJump
unsafe extern "C" fn expression_link_AscendJump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//AscendAirJump
unsafe extern "C" fn expression_link_AscendAirJump(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackm"), 0);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//AscendStart
unsafe extern "C" fn expression_link_AscendStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {  
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//Ascend
unsafe extern "C" fn expression_link_Ascend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {  
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_grapple"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32); 
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);  
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//AscendEnd
unsafe extern "C" fn expression_link_AscendEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_jump"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//RevaliGlideTurn
unsafe extern "C" fn expression_link_RevaliGlideTurn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        }
        wait(fighter.lua_state_agent, 25.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohits"), 4, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
    }
}

//RevaliGlideDrop
unsafe extern "C" fn expression_link_RevaliGlideDrop(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        ItemModule::set_attach_item_visibility(fighter.module_accessor, false, 0);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
}

//RevaliGlideLanding
unsafe extern "C" fn expression_link_RevaliGlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_normal") as i64);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_normal") as i64);
    }
}

//AttachWall
unsafe extern "C" fn expression_link_AttachWall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

//AttachWallClimb
unsafe extern "C" fn expression_link_AttachWallClimb(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
    frame(fighter.lua_state_agent, 1.0);
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_run"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        }
        wait(fighter.lua_state_agent, 1.0);
    }
}

pub fn install() {
    Agent::new("link")
    .expression_acmd("expression_ascendjump", expression_link_AscendJump)
    .expression_acmd("expression_ascendairjump", expression_link_AscendAirJump)
    .expression_acmd("expression_ascendstart", expression_link_AscendStart)
    .expression_acmd("expression_ascend", expression_link_Ascend)
    .expression_acmd("expression_ascendend", expression_link_AscendEnd)
    .expression_acmd("expression_revaliglideturn", expression_link_RevaliGlideTurn)
    .expression_acmd("expression_revaliglidedrop", expression_link_RevaliGlideDrop)
    .expression_acmd("expression_revaliglidelanding", expression_link_RevaliGlideLanding)
    .expression_acmd("expression_attachwall", expression_link_AttachWall)
    .expression_acmd("expression_attachwallclimb", expression_link_AttachWallClimb)
    .install();
}