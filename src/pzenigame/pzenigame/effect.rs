use crate::imports::BuildImports::*;

//SpecialZ
unsafe extern "C" fn effect_pzenigame_SpecialZ(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 3.8, 0, 0.0, 100.0, 0, 1.8, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 2.0, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 10.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.6, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 5.0, z: 5.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.9, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 5.5, z: 10.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.8, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.0, z: 15.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.7, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 20.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.6, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 20.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.6, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.0, z: 25.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.5, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.0, z: 30.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.4, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 35.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.8, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.5, z: 35.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.3, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 8.0, z: 40.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.2, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 8.5, z: 45.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.15, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 50.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 2.0, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.0, z: 60.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_drown_out"), false, false);
    }
}

//SpecialAirZ
unsafe extern "C" fn effect_pzenigame_SpecialAirZ(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 3.8, 0, 0.0, 100.0, 0, 1.8, true);
        let effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 4.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0}, 1.5, true, 0, 0, 0, 0, 0, true, true) as u32;
        EffectModule::set_rot(fighter.module_accessor, effect, &Vector3f { x: 0.0, y: 0.0, z: 135.0 * lr });
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 0.0, z: 10.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 2.0, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 5.0, z: 5.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.9, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 5.5, z: 10.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.8, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.0, z: 15.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.7, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 0.0, z: 20.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.6, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 20.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.6, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.0, z: 25.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.5, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.0, z: 30.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.4, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: -10.0, z: 35.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 1.8, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 7.5, z: 35.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.3, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 8.0, z: 40.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.2, true, 0, 0, 0, 0, 0, true, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 8.5, z: 45.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.15, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: -20.0, z: 50.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, 2.0, true, 0, 0, 0, 0, 0, true, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 36.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.0, z: 60.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_drown_out"), false, false);
    }
}

pub fn install() {
    Agent::new("pzenigame")
    .effect_acmd("effect_specialz", effect_pzenigame_SpecialZ, Low)
    .effect_acmd("effect_specialairz", effect_pzenigame_SpecialAirZ, Low)
    .install();
}