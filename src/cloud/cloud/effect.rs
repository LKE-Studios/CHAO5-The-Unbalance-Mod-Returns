use crate::imports::BuildImports::*;

//AttackAirF
unsafe extern "C" fn effect_cloud_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 10, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1.0, true);
        LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter, 0.2);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("cloud_airf_slash"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, true);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_CLOUD_INSTANCE_WORK_ID_INT_EFFECT_KIND_SWORD_FLARE, false, true);
    }
}

//SpecialLwLoop
unsafe extern "C" fn effect_cloud_SpecialLwLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT_OFF_KIND(fighter, Hash40::new("cloud_limitcharge_hold"), false, false);
            EFFECT_FOLLOW(fighter, Hash40::new("cloud_limitcharge_hold"), Hash40::new("top"), 1, 10, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("cloud_limitcharge_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

//SpecialAirLwLoop
unsafe extern "C" fn effect_cloud_SpecialAirLwLoop(fighter: &mut L2CAgentBase) {
    loop {
        if is_excute(fighter) {
            EFFECT_FOLLOW(fighter, Hash40::new("cloud_limitcharge_hold"), Hash40::new("top"), 1, 10, 0, 0, 0, 0, 1, true);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
        wait(fighter.lua_state_agent, 9.0);
        wait_loop_clear(fighter);
    }
}

pub fn install() {
    Agent::new("cloud")
    .effect_acmd("effect_attackairf", effect_cloud_AttackAirF, Low)
    .effect_acmd("effect_speciallwloop", effect_cloud_SpecialLwLoop, Low)
    .effect_acmd("effect_specialairlwloop", effect_cloud_SpecialAirLwLoop, Low)
    .install();
}
