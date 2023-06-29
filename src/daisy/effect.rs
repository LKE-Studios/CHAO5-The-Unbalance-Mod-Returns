use crate::imports::BuildImports::*;

#[acmd_script(//AttackLw3
    agent = "daisy", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -4, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0.0, 2.0, 7.0, 0.0, 20.0, 0.0, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
}

#[acmd_script(//AttackAirN
    agent = "daisy", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 1.5, 7.5, 0.0, 0.0, -80.0, -105, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 15, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true, 0.7);
    }
}

#[acmd_script(//AttackAirF
    agent = "daisy", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0.0, 10.0, 0.0, -4.0, -55.0, 96.0, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toel"), 0.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_d"), true, true);
    }
}

#[acmd_script(//AttackAirHi
    agent = "daisy", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc"), Hash40::new("sys_attack_arc"), Hash40::new("top"), 0.0, 9.0, 0.0, -4.0, 20.0, 96.0, 0.95, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.9, /*B*/ 0.04);
    }
}

#[acmd_script(//AttackAirLw
    agent = "daisy", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), 0.0, 10.0, -1.5, 80.0, 0.0, 0.0, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.4)
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_impact"), Hash40::new("sys_attack_impact"), Hash40::new("top"), 0.0, -5.0, 0.2, 80.0, 0.0, 0.0, 1.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.4)
    }
}

#[acmd_script(//Shot
    agent = "daisy_kinopiospore", 
    script = "effect_shot", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_daisy_kinopiospore_shot(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("daisy_kinopio_bullet"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 6.0, true);
    }
} 

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_daisy_attacklw3,
        effect_daisy_attackairn,
        effect_daisy_attackairf,
        effect_daisy_attackairhi,
        effect_daisy_attackairlw, 
        effect_daisy_kinopiospore_shot
    );
}