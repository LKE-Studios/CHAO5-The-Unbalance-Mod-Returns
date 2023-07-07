use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "expression_glidestart", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_plizardon_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
}

#[acmd_script(//SpecialAirHi2Start
    agent = "plizardon", 
    script = "expression_specialairhi2start", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_plizardon_specialairhi2start(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_jump"), 0, false, 0);
    }
}

#[acmd_script(//SpecialHi2
    agent = "plizardon", 
    script = "expression_plizardon_specialairhi2", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_plizardon_specialairhi2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialAirHi2Landing
    agent = "plizardon", 
    script = "expression_specialairhi2landing", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_plizardon_specialairhi2landing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohit_attacks"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_plizardon_glidestart,
        expression_plizardon_specialairhi2start,
        expression_plizardon_specialairhi2,
        expression_plizardon_specialairhi2landing
    );
}