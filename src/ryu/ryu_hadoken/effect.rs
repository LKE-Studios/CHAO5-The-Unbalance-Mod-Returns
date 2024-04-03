use crate::imports::BuildImports::*;

//MoveS
unsafe extern "C" fn effect_ken_hadoken_MoveS(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
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

//MoveM
unsafe extern "C" fn effect_ken_hadoken_MoveM(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
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

//MoveW
unsafe extern "C" fn effect_ken_hadoken_MoveW(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *WEAPON_RYU_HADOKEN_INSTANCE_WORK_ID_FLAG_COMMAND) {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
        }
    }
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullethand_l"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.0, false);
        }
    }
    else {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("ken_hadoken_bullet2"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 1.5, false);
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
    Agent::new("ken_hadoken")
    .effect_acmd("effect_moves", effect_ken_hadoken_MoveS)
    .effect_acmd("effect_movem", effect_ken_hadoken_MoveM)
    .effect_acmd("effect_movew", effect_ken_hadoken_MoveW)
    .install();
}