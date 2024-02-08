use crate::imports::BuildImports::*;

#[acmd_script(//SpecialLwHold
    agent = "ness", 
    script = "expression_speciallwhold_ninten", 
    category = ACMD_GAME )]
unsafe fn expression_ninten_speciallwhold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter,0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
}

#[acmd_script(//SpecialAirLwHold
    agent = "ness", 
    script = "expression_specialairlwhold_ninten", 
    category = ACMD_GAME )]
unsafe fn expression_ninten_specialairlwhold(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        AREA_WIND_2ND_arg10(fighter,0, 2, 225, 2, 1, 0, 12, 30, 30, 80);
    }
}

#[acmd_script(//SpecialLwHit
    agent = "ness", 
    script = "expression_speciallwhit_ninten", 
    category = ACMD_EXPRESSION )]
unsafe fn expression_ninten_speciallwhit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//SpecialAirLwHit
    agent = "ness", 
    script = "expression_specialairlwhit_ninten", 
    category = ACMD_EXPRESSION )]
unsafe fn expression_ninten_specialairlwhit(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_shield_off"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

#[acmd_script(//AppealSR
    agent = "ness", 
    script = "expression_appealsr_ninten", 
    category = ACMD_GAME )]
unsafe fn expression_ninten_appealsr(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

#[acmd_script(//AppealSL
    agent = "ness", 
    script = "expression_appealsl_ninten", 
    category = ACMD_GAME )]
unsafe fn expression_ninten_appealsl(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        expression_ninten_speciallwhold,
        expression_ninten_specialairlwhold,
        expression_ninten_speciallwhit,
        expression_ninten_specialairlwhit,
        expression_ninten_appealsr,
        expression_ninten_appealsl,
    );
}