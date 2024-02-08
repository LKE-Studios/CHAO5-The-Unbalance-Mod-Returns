use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "metaknight", 
    script = "expression_glidestart", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_metaknight_glidestart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
}

#[acmd_script(//GlideDirection
    agent = "metaknight", 
    script = "expression_glidedirection", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_metaknight_glidedirection(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
}

#[acmd_script(//SpecialHi
    agent = "metaknight", 
    script = "expression_specialhi", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_metaknight_specialhi(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("haver"), AttackDirectionAxis(*ATTACK_DIRECTION_Z), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_X));
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_normal") as i64);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_slashm"), 0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        VisibilityModule::set_status_default_int64(fighter.module_accessor, hash40("mantle") as i64, hash40("mantle_wing") as i64);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitm"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_metaknight_glidestart,
        expression_metaknight_glidedirection,
        expression_metaknight_specialhi
    );
}