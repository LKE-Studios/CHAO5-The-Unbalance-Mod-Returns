use crate::imports::BuildImports::*;

#[acmd_script(//AttachWall
    agent = "link", 
    script = "expression_attachwall", 
    category = ACMD_EXPRESSION)]
unsafe fn expression_link_attachwall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, hash40("shield") as i64, hash40("shield_back") as i64);
        VisibilityModule::set_int64(fighter.module_accessor, hash40("sword") as i64, hash40("sword_back") as i64);
    }
}

#[acmd_script(//AttachWallClimb
    agent = "link", 
    script = "expression_attachwallclimb", 
    category = ACMD_EXPRESSION)]
unsafe fn expression_link_attachwallclimb(fighter: &mut L2CAgentBase) {
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
    install_acmd_scripts!(
        expression_link_attachwall,
        expression_link_attachwallclimb
    );
}