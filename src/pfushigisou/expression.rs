use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecialCharge
    agent = "pfushigisou", 
    script = "expression_guardspecialcharge", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_guardspecialcharge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//GuardSpecialShoot
    agent = "pfushigisou", 
    script = "expression_guardspecialshoot", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_guardspecialshoot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosionl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
    frame(fighter.lua_state_agent, 56.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 4);
    }
}

#[acmd_script(//GuardSpecialEnd
    agent = "pfushigisou", 
    script = "expression_guardspecialend", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_guardspecialend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

#[acmd_script(//EscapeAirSpecialCharge
    agent = "pfushigisou", 
    script = "expression_escapeairspecialcharge", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_escapeairspecialcharge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//EscapeAirSpecialShoot
    agent = "pfushigisou", 
    script = "expression_escapeairspecialshoot", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_escapeairspecialshoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
    }
}

#[acmd_script(//EscapeAirSpecialEnd
    agent = "pfushigisou", 
    script = "expression_escapeairspecialend", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pfushigisou_escapeairspecialend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_R);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 5);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_pfushigisou_guardspecialcharge,
        expression_pfushigisou_guardspecialshoot,
        expression_pfushigisou_guardspecialend,
        expression_pfushigisou_escapeairspecialcharge,
        expression_pfushigisou_escapeairspecialshoot,
        expression_pfushigisou_escapeairspecialend
    );
}