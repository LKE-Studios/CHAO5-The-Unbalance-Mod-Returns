use crate::imports::BuildImports::*;

#[acmd_script(//Attack13
    agent = "lucina", 
    script = "effect_attack13", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);

    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, 6, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script(//AttackS3Hi
    agent = "lucina", 
    script = "effect_attacks3hi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacks3hi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script(//AttackS3
    agent = "lucina", 
    script = "effect_attacks3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script(//AttackS3Lw
    agent = "lucina", 
    script = "effect_attacks3lw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacks3lw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script(//AttackLw3 
    agent = "lucina", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 90.0, 0.0, 90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 3, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 2);
    }
}

#[acmd_script(//AttackS4
    agent = "lucina", 
    script = "effect_attacks4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacks4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -0.0, 8, 5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 90.0, 0.0, 90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 8, 0, 0, 0, 180, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script(//AttackHi4
    agent = "lucina", 
    script = "effect_attackhi4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 90.0, 0.0, 90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        LAST_PARTICLE_SET_COLOR(fighter, 0.4, 0.6, 1);
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.15, 0, 10, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 1);
    }
}

#[acmd_script(//AttackLw4
    agent = "lucina",
    script = "effect_attacklw4", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sword1"), -0.0, -0.0, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 12, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_light"), Hash40::new("sword1"), 0, 0, 10, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(fighter.lua_state_agent, 21.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_light"), false, true);
    }
}

#[acmd_script(//AttackAirN
    agent = "lucina", 
    script = "effect_attackairn", 
    category = ACMD_EFFECT, 
    low_priority)]
unsafe fn effect_lucina_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 8, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
        EFFECT_FOLLOW(fighter, Hash40::new("lucina_sword_light"), Hash40::new("sword1"), 0, 0, 11, 0, 0, 0, 0.4, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("lucina_sword_light"), false, true);
    }
}

#[acmd_script(//AttackAirF
    agent = "lucina", 
    script = "effect_attackairf", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script(//AttackAirB
    agent = "lucina", 
    script = "effect_attackairb", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 4, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -0.0, 7.5, -6.0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.264, 0.47, 1.3);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_sp_flash"), Hash40::new("sword1"), -0.0, -0.0, 10.0, 0, 0, 0, 0.6, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script(//AttackAirHi
    agent = "lucina", 
    script = "effect_attackairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 6, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 3);
    }
}

#[acmd_script(//AttackAirLw
    agent = "lucina", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 5, Hash40::new("sword1"), 0.0, 0.0, 1.65, Hash40::new("sword1"), -0.0, -0.0, 12.4, true, Hash40::new("lucina_sword"), Hash40::new("sword1"), 0.0, 0.0, 0.0, 90.0, 0.0, 90.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.2, 0.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 5);
    }
}

#[acmd_script(//SpecialLwHit2
    agent = "lucina", 
    script = "effect_speciallwhit2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_speciallwhit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_guard_mark"), true, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.25, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

#[acmd_script(//SpecialAirLwHit2
    agent = "lucina", 
    script = "effect_specialairlwhit2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_lucina_specialairlwhit2(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_guard_mark"), true, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_lucina_sword1"), Hash40::new("tex_lucina_sword2"), 10, Hash40::new("sword1"), 0.0, 0.0, 1.7, Hash40::new("sword1"), -0.0, -0.0, 12.6, true, Hash40::new("lucina_sword"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.4, 0.2);
    }
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        FLASH(fighter, 1, 1, 1, 0);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        AFTER_IMAGE_OFF(fighter, 4);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_lucina_attack13,
        effect_lucina_attacks3hi,
        effect_lucina_attacks3,
        effect_lucina_attacks3lw,
        effect_lucina_attacklw3,
        effect_lucina_attacks4,
        effect_lucina_attackhi4,
        effect_lucina_attacklw4,
        effect_lucina_attackairn,
        effect_lucina_attackairf,
        effect_lucina_attackairb,
        effect_lucina_attackairhi,
        effect_lucina_attackairlw,
        effect_lucina_speciallwhit2,
        effect_lucina_specialairlwhit2
    );
}