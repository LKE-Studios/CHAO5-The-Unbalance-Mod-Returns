use crate::imports::BuildImports::*;

//Final 
unsafe extern "C" fn effect_silver_mewtwom_Final(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_aura"), Hash40::new("head"), 0, -3, 0, 0, 0, 0, 0.6, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_aura"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 0.7, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
}

//FinalEnd 
unsafe extern "C" fn effect_silver_mewtwom_FinalEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_final_shot"), false, false);
        BURN_COLOR(fighter, 1, 0.0, 1.0, 1.0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 5, 1, 1.0, 1.0, 1);
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
}

//FinalShoot
unsafe extern "C" fn effect_silver_mewtwom_FinalShoot(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("mewtwo_final_shot_hold"), Hash40::new("arml"), 4, 0, -2, 0, 0, 0, 1, false);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_final_shot_hold"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("0x161efe0679"), false, false);
        EFFECT(fighter, Hash40::new("mewtwo_final_shot"), Hash40::new("top"), 0, 13, 8, 0, 0, 0, 0.6, 0, 0, 0, 50, 360, 50, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 1.0, 1.0);
    }
    for _ in 0..90 {
        wait(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        }
    }
}

pub fn install() {
    Agent::new("mewtwo_mewtwom")
    .effect_acmd("effect_final_silver", effect_silver_mewtwom_Final, Low)
    .effect_acmd("effect_finalend_silver", effect_silver_mewtwom_FinalEnd, Low)
    .effect_acmd("effect_finalshoot_silver", effect_silver_mewtwom_FinalShoot, Low)
    .install();
}