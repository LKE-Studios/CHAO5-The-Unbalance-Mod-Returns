use crate::imports::BuildImports::*;

#[acmd_script(//Attack11
    agent = "claus", 
    script = "effect_attack11" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FLIP(fighter, Hash40::new("sys_attack_line"), Hash40::new("sys_attack_line"), Hash40::new("top"), -4.0, 4.5, 0, 0, 0, 0, 0.9, 0, 1, 0, 0, 0, 0, false, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("handr"), 3.0, 0.1, 0.0, 0, 0, 0, 0.35, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
}

#[acmd_script(//Attack12
    agent = "claus", 
    script = "effect_attack12" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attack12(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("haver"), -0.2, 0, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("handr"), 3.1, 0.2, 0.0, 0, 0, 0, 0.36, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
    }
}

#[acmd_script(//AttackHi3
    agent = "claus", 
    script = "effect_attackhi3" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attackhi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 13.5, 2.0, 0, 45, 90, 0.85, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4.5, 0.2, 0, 0, 0, 0, 0.5, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//AttackLw3
    agent = "claus", 
    script = "effect_attacklw3" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("sys_attack_arc_d"), Hash40::new("sys_attack_arc_d"), Hash40::new("top"), 0, 2.0, 7.0, 0, 20, 0, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.0, /*G*/ 1.9, /*B*/ 0.5);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("kneer"), 4.0, 0.1, -1.0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
}

#[acmd_script(//AttackLw4
    agent = "claus", 
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_claus_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0.5, 0.5, 1.3, 0, 0, 0, 1.0, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("lucas_psi_atk_lw"), Hash40::new("lucas_psi_atk_lw"), Hash40::new("top"), 0, 2.5, 8, 0, 0, 0, 2.1, true, *EF_FLIP_YZ);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -3, 0, 0, 0, 0, 0, 0.7, 2, 2, 2, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}

#[acmd_script(//AttackAirN
    agent = "claus", 
    script = "effect_attackairn" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attackairn(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_hold"), Hash40::new("havel"), 0, 0, 1, 0, 0, 0, 0.9, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_hold"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 4.5, 0, -90, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1, 3.5, 0, -90, 0, 0, 1.1, true);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 1.5, 3.5, 0, -90, 0, 0, 1.41, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
}

#[acmd_script(//AttackAirLw
    agent = "claus", 
    script = "effect_attackairlw" , 
    category = ACMD_EFFECT , 
    low_priority)]
unsafe fn effect_claus_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0.0, -4, 0.0, 0, 0, 0, 0.81, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.65, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    wait(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.7, true);
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucas_psi_atk"), false, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("lucas_psi_atk"), Hash40::new("top"), 0.0, -4.0, 0.3, 0, 90, 0, 0.84, true);
        LAST_EFFECT_SET_RATE(fighter, 1.3);
    }
}

#[acmd_script(//WessDance
    agent = "claus", 
    script = "effect_wessdance", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_claus_wessdance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1370.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
    wait(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 22.0, 0, 0, 0, 0, 0.65, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_claus_attack11,
        effect_claus_attack12,
        effect_claus_attackhi3,
        effect_claus_attacklw3,
        effect_claus_attacklw4,
        effect_claus_attackairn,
        effect_claus_attackairlw,
        effect_claus_wessdance
    );
}