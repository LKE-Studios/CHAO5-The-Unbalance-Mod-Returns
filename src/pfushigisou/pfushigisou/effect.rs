use crate::imports::BuildImports::*;

//SpecialAirHi
unsafe extern "C" fn effect_pfushigisou_SpecialAirHi(fighter: &mut L2CAgentBase) {
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

//SpecialZStart
unsafe extern "C" fn effect_pfushigisou_SpecialZStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_speed_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
}

//SpecialZCharge
unsafe extern "C" fn effect_pfushigisou_SpecialZCharge(fighter: &mut L2CAgentBase) {
    loop {
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

//SpecialZEnd
unsafe extern "C" fn effect_pfushigisou_SpecialZEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_speed_up"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_recovery"), false, true);
    }
}

//SpecialZShoot
unsafe extern "C" fn effect_pfushigisou_SpecialZShoot(fighter: &mut L2CAgentBase) {
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

//SpecialAirZStart
unsafe extern "C" fn effect_pfushigisou_SpecialAirZStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_status_speed_up"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.8, true);
    }
}

//SpecialAirZCharge
unsafe extern "C" fn effect_pfushigisou_SpecialAirZCharge(fighter: &mut L2CAgentBase) {
    loop {
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

//SpecialAirZShoot
unsafe extern "C" fn effect_pfushigisou_SpecialAirZShoot(fighter: &mut L2CAgentBase) {
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

//SpecialAirZEnd
unsafe extern "C" fn effect_pfushigisou_SpecialAirZEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        COL_NORMAL(fighter);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_status_speed_up"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_recovery"), false, true);
    }
}

pub fn install() {
    Agent::new("pfushigisou")
    .effect_acmd("effect_specialairhi", effect_pfushigisou_SpecialAirHi, Low)
    .effect_acmd("effect_specialzstart", effect_pfushigisou_SpecialZStart, Low)
    .effect_acmd("effect_specialzcharge", effect_pfushigisou_SpecialZCharge, Low)
    .effect_acmd("effect_specialzend", effect_pfushigisou_SpecialZEnd, Low)
    .effect_acmd("effect_specialzshoot", effect_pfushigisou_SpecialZShoot, Low)
    .effect_acmd("effect_specialairzstart", effect_pfushigisou_SpecialAirZStart, Low)
    .effect_acmd("effect_specialairzcharge", effect_pfushigisou_SpecialAirZCharge, Low)
    .effect_acmd("effect_specialairzend", effect_pfushigisou_SpecialAirZEnd, Low)
    .effect_acmd("effect_specialairzshoot", effect_pfushigisou_SpecialAirZShoot, Low)
    .install();
}