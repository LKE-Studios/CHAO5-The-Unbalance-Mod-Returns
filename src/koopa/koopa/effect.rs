use crate::imports::BuildImports::*;

//AttackLw3 
unsafe extern "C" fn effect_koopa_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 6, 6, 7, -34, -20, 1.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 6, 8, -4, -59, -167, 1.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2.3);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackAirLw 
unsafe extern "C" fn effect_koopa_AttackAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -5, -5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("koopa_atk_fall"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.65, true, 1, 1, 1);
        smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

//SpecialSCatch
unsafe extern "C" fn effect_koopa_SpecialSCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
        if is_excute(fighter) {
            EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 2.5, -90, -204, 1.8, true, 0.1);
        }
        else {
            if is_excute(fighter) {
                EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 18.7, -90, 2, 1.8, true, 0.1);
            }
        }
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        FLASH(fighter, 1, 1, 0.753, 0.627);
        FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialSAirCatch
unsafe extern "C" fn effect_koopa_SpecialSAirCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 18.7, -90, 2, 1.8, true, *EF_FLIP_YZ, 0.1);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 0.753, 0.627);
        FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
}

//SpecialLw 
unsafe extern "C" fn effect_koopa_SpecialLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("koopa_drop_arc"), Hash40::new("koopa_drop_arc"), Hash40::new("top"), -7, 15, 8, 12.46, -45.812, 61.035, 1.15, true, *EF_FLIP_YZ, 0.7);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("koopa_drop_arc"), true, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("koopa_drop"), Hash40::new("top"), 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 1.0, false, 0.941, 1.0, 0.863);
    }
}

//SpecialAirLw 
unsafe extern "C" fn effect_koopa_SpecialAirLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_COLOR(fighter, Hash40::new("koopa_drop_air"), Hash40::new("top"), 0.0, 12.0, 0.0, 0.0, 0.0, 0.0, 1.0, true, 0.941, 1.0, 0.863);
    }
}

pub fn install() {
    Agent::new("koopa")
    .effect_acmd("effect_attacklw3", effect_koopa_AttackLw3)
    .effect_acmd("effect_attackairlw", effect_koopa_AttackAirLw)
    .effect_acmd("effect_specialscatch", effect_koopa_SpecialSCatch)
    .effect_acmd("effect_specialsaircatch", effect_koopa_SpecialSAirCatch)
    .effect_acmd("effect_speciallw", effect_koopa_SpecialLw)
    .effect_acmd("effect_specialairlw", effect_koopa_SpecialAirLw)
    .install();
}