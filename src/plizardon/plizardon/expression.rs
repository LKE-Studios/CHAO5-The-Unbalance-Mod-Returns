use crate::imports::BuildImports::*;

//GlideStart
unsafe extern "C" fn expression_plizardon_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_dash"), 0, false, 0);
    }
}

//GlideAttack
unsafe extern "C" fn expression_plizardon_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_attackm"), 0, false, 0);
    }
}

//GlideLanding
unsafe extern "C" fn expression_plizardon_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_landing"), 0, false, 0);
    }
}

//SpecialAirHi2Start
unsafe extern "C" fn expression_plizardon_SpecialAirHi2Start(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_jump"), 0, false, 0);
    }
}

//SpecialHi2
unsafe extern "C" fn expression_plizardon_SpecialAirHi2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackll"), 0);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_spinattack"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

//SpecialAirHi2Landing
unsafe extern "C" fn expression_plizardon_SpecialAirHi2Landing(fighter: &mut L2CAgentBase) {
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

pub unsafe extern "C" fn expression_plizardon_SpecialZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

pub unsafe extern "C" fn expression_plizardon_SpecialAirZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_nohitl"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        QUAKE(fighter, *CAMERA_QUAKE_KIND_M);
        RUMBLE_HIT(fighter, Hash40::new("rbkind_attackl"), 0);
    }
}

pub fn install() {
    Agent::new("plizardon")
    .expression_acmd("expression_glidestart", expression_plizardon_GlideStart, Low)
    .expression_acmd("expression_glideattack", expression_plizardon_GlideAttack, Low)
    .expression_acmd("expression_glidelanding", expression_plizardon_GlideLanding, Low)
    .expression_acmd("expression_specialairhi2start", expression_plizardon_SpecialAirHi2Start, Low)
    .expression_acmd("expression_specialairhi2", expression_plizardon_SpecialAirHi2, Low)
    .expression_acmd("expression_specialairhi2landing", expression_plizardon_SpecialAirHi2Landing, Low)
    .expression_acmd("expression_specialz", expression_plizardon_SpecialZ, Low)
    .expression_acmd("expression_specialairz", expression_plizardon_SpecialAirZ, Low)
    .install();
}