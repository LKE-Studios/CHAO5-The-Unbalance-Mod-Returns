use crate::imports::BuildImports::*;

//AttackS4
unsafe extern "C" fn effect_snake_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4, 13, 8, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 40.0);
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_S4_IS_CHARGED) {
        frame(fighter.lua_state_agent, 41.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("snake_atk_s4s_shot"), Hash40::new("haver"), 0, 1.4, -4, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 16, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 16, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 16, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, -1, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 16, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 40, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 40, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 40, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 40, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 65, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 65, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 65, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 65, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 90, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 90, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 90, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 90, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 115, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 115, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 115, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 90, 0, 115, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
        wait(fighter.lua_state_agent, 3.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 140, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 140, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 140, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 90, 0, 140, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
        }
    }
    else {
        frame(fighter.lua_state_agent, 41.0);
        if is_excute(fighter) {
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("snake_atk_s4s_shot"), Hash40::new("haver"), 0, 1.4, -4, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.8);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb1"), Hash40::new("top"), 0, -2, 16, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb_smoke"), Hash40::new("top"), 0, -2, 16, 0, 180, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
            EFFECT(fighter, Hash40::new("snake_atk_s4s_bomb2"), Hash40::new("top"), 0, -2, 16, 0, 90, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
            LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 2, -1, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
            EFFECT(fighter, Hash40::new("snake_atk_s4s"), Hash40::new("top"), 0, 0, 16, 0, -90, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
    }
}

pub fn install() {
    Agent::new("snake")
    .effect_acmd("effect_attacks4", effect_snake_AttackS4)
    .install();
}