use crate::imports::BuildImports::*;

#[acmd_script(//AttackLw4
    agent = "lucas", 
    script = "expression_attacklw4", 
    category = ACMD_EXPRESSION, 
    low_priority)]
unsafe fn expression_claus_attacklw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
    }
    frame(fighter.lua_state_agent, 6.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) == true {
        if is_excute(fighter) {
            ItemModule::set_have_item_visibility(fighter.module_accessor, false, 0);
            slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        }
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_beams"), 0, false, 0);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        AREA_WIND_2ND_RAD_arg9(fighter, 1, 0.1, 0.2, 3, 0.2, 9, 4, 18, 80);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        AreaModule::erase_wind(fighter.module_accessor, 1);
        ItemModule::set_have_item_visibility(fighter.module_accessor, true, 0);
    }
}



pub fn install() {
    smashline::install_acmd_scripts!(
        expression_claus_attacklw4
    );
}