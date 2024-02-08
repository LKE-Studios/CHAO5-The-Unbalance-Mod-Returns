use crate::imports::BuildImports::*;

#[acmd_script(//MoveS
    agent = "ken",
    script = "effect_moves",
    category = ACMD_EFFECT,
    low_priority )]
unsafe fn effect_ken_moves(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 3.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 4.0, false);
        }
    } 
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
        else {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
            }
        }
    }
}

#[acmd_script(//MoveM
    agent = "ken",
    script = "effect_movem",
    category = ACMD_EFFECT,
    low_priority )]
unsafe fn effect_ken_movem(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 3.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 4.0, false);
        }
    } 
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
        else {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
            }
        }
    }
}

#[acmd_script(//MoveW
    agent = "ken",
    script = "effect_movew",
    category = ACMD_EFFECT,
    low_priority )]
unsafe fn effect_ken_movew(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) == false {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 3.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 4.0, false);
        }
    } 
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
        else {
            if is_excute(fighter) {
                EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_r"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
            }
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_ken_moves,
        effect_ken_movem,
        effect_ken_movew
    );
}