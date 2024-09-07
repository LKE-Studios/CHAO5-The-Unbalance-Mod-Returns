use crate::imports::BuildImports::*;

#[acmd_script(//SpecialAirHi
    agent = "pfushigisou", 
    script = "effect_specialairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash_s"), Hash40::new("top"), 0, 6.5, 5, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 1.5)
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner5"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner4"), 0, 0, 0, 0, 0, 0, 1.2, true, 0.7);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("pfushigisou_vine"), Hash40::new("viner3"), 0, 0, 0, 0, 0, 0, 1.2, true, 0.5);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_a1"), Hash40::new("tex_pfushigisou_vine_sp_a2"), 4, Hash40::new("viner5"), -1.0, 0.0, 0.0, Hash40::new("viner5"), 6.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("viner5"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_b1"), Hash40::new("tex_pfushigisou_vine_sp_b2"), 4, Hash40::new("viner4"), 0.0, 0.0, 0.0, Hash40::new("viner4"), 6.0, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("viner4"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
        AFTER_IMAGE4_ON_arg29(fighter, Hash40::new("tex_pfushigisou_vine_sp_b1"), Hash40::new("tex_pfushigisou_vine_sp_b2"), 4, Hash40::new("viner3"), 2.0, 0.0, 0.0, Hash40::new("viner3"), 6.5, 0.0, 0.0, true, Hash40::new("null"), Hash40::new("viner3"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0, *EFFECT_AXIS_X, 0, *TRAIL_BLEND_ALPHA, 101, *TRAIL_CULL_NONE, 1.3, 0.2);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("pfushigisou_vine"), false, true);
        AFTER_IMAGE_OFF(fighter,3);
    }
}    

#[acmd_script(//GuardSpecialStart
    agent = "pfushigisou", 
    script = "effect_guardspecialstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_guardspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_speed_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
}

#[acmd_script(//GuardSpecialCharge
    agent = "pfushigisou", 
    script = "effect_guardspecialcharge", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_guardspecialcharge(fighter: &mut L2CAgentBase) {
    for _ in 0..6 {
        if is_excute(fighter) {
            FLASH(fighter, 0.0, 1.0, 0.14, 0.6);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            FLASH(fighter, 0.0, 0.0, 0.0, 0.0);
        }
    }
}

#[acmd_script(//GuardSpecialEnd
    agent = "pfushigisou", 
    script = "effect_guardspecialend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_guardspecialend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_speed_up"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_recovery"), false, true);
    }
}

#[acmd_script(//GuardSpecialShoot
    agent = "pfushigisou", 
    script = "effect_guardspecialshoot", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_guardspecialshoot(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("finptrainer_solar_beam"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.5, z: 7.7 }, &Vector3f { x: 0.0, y: 0.0, z: 45.0 * lr }, 1.0, true, 0, 0, 0, 0, 0, true, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("finptrainer_solar_beam"), false, false);
    }
}

#[acmd_script(//EscapeAirSpecialStart
    agent = "pfushigisou", 
    script = "effect_escapeairspecialstart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_escapeairspecialstart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_speed_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
}

#[acmd_script(//EscapeAirSpecialCharge
    agent = "pfushigisou", 
    script = "effect_escapeairspecialcharge", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_escapeairspecialcharge(fighter: &mut L2CAgentBase) {
    for _ in 0..6 {
        if is_excute(fighter) {
            FLASH(fighter, 0.0, 1.0, 0.14, 0.6);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(fighter.lua_state_agent, 20.0);
        if is_excute(fighter) {
            COL_NORMAL(fighter);
            FLASH(fighter, 0.0, 0.0, 0.0, 0.0);
        }
    }
}

#[acmd_script(//EscapeAirSpecialShoot
    agent = "pfushigisou", 
    script = "effect_escapeairspecialshoot", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_escapeairspecialshoot(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("finptrainer_solar_beam"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.5, z: 7.7 }, &Vector3f { x: 0.0, y: 0.0, z: 45.0 * lr }, 1.0, true, 0, 0, 0, 0, 0, true, true);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -5.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true);
    }
    wait(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("finptrainer_solar_beam"), false, false);
    }
}

#[acmd_script(//EscapeAirSpecialEnd
    agent = "pfushigisou", 
    script = "effect_escapeairspecialend", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pfushigisou_escapeairspecialend(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_speed_up"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_recovery"), false, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_pfushigisou_specialairhi,
        effect_pfushigisou_guardspecialstart,
        effect_pfushigisou_guardspecialcharge,
        effect_pfushigisou_guardspecialend,
        effect_pfushigisou_guardspecialshoot,
        effect_pfushigisou_escapeairspecialstart,
        effect_pfushigisou_escapeairspecialcharge,
        effect_pfushigisou_escapeairspecialshoot,
        effect_pfushigisou_escapeairspecialend
    );
}