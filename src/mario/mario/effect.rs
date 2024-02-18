use crate::imports::BuildImports::*;

//AttackS4
unsafe extern "C" fn effect_mario_AttackS4(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 { //Ice Mario on c09
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 0, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 8, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    }
}

//AttackS4Hi
unsafe extern "C" fn effect_mario_AttackS4Hi(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 { //Ice Mario on c09
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, -15, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_freeze"), Hash40::new("top"), 0, 11, 12, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, -15, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 11, 12, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    }
}

//AttackS4Lw
unsafe extern "C" fn effect_mario_AttackS4Lw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 { //Ice Mario on c09
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 18, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_freezer"), Hash40::new("top"), 0, 4, 11, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    } 
    else {
        frame(fighter.lua_state_agent, 1.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 2.0, 3, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 14.0);
        if is_excute(fighter) {
            EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 1, 7, -1, 18, -2, 0, 0.8, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_shoot"), Hash40::new("havel"), 0, 0, 0, 0, -45, 0, 1, true);
        }
        frame(fighter.lua_state_agent, 15.0);
        if is_excute(fighter) {
            EFFECT(fighter, Hash40::new("sys_flame"), Hash40::new("top"), 0, 4, 11, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        }
        frame(fighter.lua_state_agent, 16.0);
        if is_excute(fighter) {
            LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 22.0);
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, true);
        }
    }
}

pub fn install() {
    Agent::new("mario")
    .effect_acmd("effect_attacks3s3", effect_mario_AttackS4Hi)
    .effect_acmd("effect_attacks4", effect_mario_AttackS4)
    .effect_acmd("effect_attackhi4", effect_mario_AttackS4Lw)
    .install();
}