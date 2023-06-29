use crate::imports::BuildImports::*;

#[acmd_script(//AttackLw3
    agent = "samusd", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attacklw3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 14.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 28.387, -0.341, -0.169, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 13.4, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 26.8, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
    }
}

#[acmd_script(//AttackLw4
    agent = "samusd", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attacklw4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, 17.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, -19.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, 8.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
        EFFECT(fighter, Hash40::new("samusd_throw_hi"), Hash40::new("top"), 0, 0, -9.4, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 39.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
    }
}

#[acmd_script(//AttackAirN
    agent = "samusd", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
        BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_impact"), Hash40::new("hip"), 0, 0, 0, 0, 0, 0, 2.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        BURN_COLOR_NORMAL(fighter);
    }
}

#[acmd_script(//AttackAirLw 
    agent = "samusd", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_attackairlw(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("shoulderr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("clavicler"), 0, 0, 0, 0, 0, 0, 1.9, true);
        EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("claviclel"), 0, 0, 0, 0, 0, 0, 1.9, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 15, 0, 90, 0, 0, 1.7, true);
        LAST_EFFECT_SET_COLOR(fighter, 3.0 / 255.0, 194.0 / 255.0, 252.0 / 255.0);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
    }
}

#[acmd_script(//HBurst 
    agent = "samusd_missile", 
    script = "effect_hburst", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_missile_hburst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script(//SBurst
    agent = "samusd_supermissile", 
    script = "effect_sburst", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_supermissile_sburst(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.04, 0, 0, 0, 0, 0, 0, true);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script(//BurstAttack
    agent = "samusd_bomb", 
    script = "effect_burstattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_samusd_bomb_burstattack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("samusd_bomb_a"), Hash40::new("samusd_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.2, 0, 0, 0, 0, 0, 0, false);
        EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.12, /*G*/ 0.1, /*B*/ 1.85);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_samusd_attacklw3,
        effect_samusd_attacklw4,
        effect_samusd_attackairn,
        effect_samusd_attackairlw,
        effect_samusd_missile_hburst,
        effect_samusd_supermissile_sburst,
        effect_samusd_bomb_burstattack
    );
}