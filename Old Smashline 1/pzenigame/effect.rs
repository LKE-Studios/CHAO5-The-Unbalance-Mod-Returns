use crate::imports::BuildImports::*;

#[acmd_script(//GuardSpecial
    agent = "pzenigame", 
    script = "effect_guardspecial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pzenigame_guardspecial(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 3.8, 0, 0.0, 100.0, 0, 1.8, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 4.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.5, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 2.0, true, 0, 0, 0, 0, 0, true, true);
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
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.0, z: 50.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_drown_out"), false, false);
    }
}

#[acmd_script(//EscapeAirSpecial
    agent = "pzenigame", 
    script = "effect_escapeairspecial", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_pzenigame_escapeairspecial(fighter: &mut L2CAgentBase) {
    let lr = PostureModule::lr(fighter.module_accessor);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_water_walk"), Hash40::new("head"), 0.0, 3.8, 0, 0.0, 100.0, 0, 1.8, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_drown_out"), Hash40::new("top"), &Vector3f { x: 0.0, y: 6.5, z: 4.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.5, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 2.0, true, 0, 0, 0, 0, 0, true, true);
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
    }
    frame(fighter.lua_state_agent, 32.0);
    if is_excute(fighter) {
        EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_sleep"), Hash40::new("top"), &Vector3f { x: 0.0, y: 9.0, z: 50.0 }, &Vector3f { x: 0.0, y: 0.0, z: 90.0 * lr }, 1.1, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_drown_out"), false, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_pzenigame_guardspecial,
        effect_pzenigame_escapeairspecial
    );
}