use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecial
    agent = "pzenigame", 
    script = "expression_guardspecial", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pzenigame_guardspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_waterjetss"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//EscapeAirSpecial
    agent = "pzenigame", 
    script = "expression_escapeairspecial", 
    category = ACMD_EXPRESSION, 
    low_priority )]
unsafe fn expression_pzenigame_escapeairspecial(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_waterjetss"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_pzenigame_guardspecial,
        expression_pzenigame_escapeairspecial
    );
}