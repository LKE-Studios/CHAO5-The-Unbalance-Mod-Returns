use crate::imports::BuildImports::*;

//JumpAerialF3 
unsafe extern "C" fn effect_buddy_JumpAerial3(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.85, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

//JumpAerialF4
unsafe extern "C" fn effect_buddy_JumpAerial4(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.85, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

//JumpAerialF5
unsafe extern "C" fn effect_buddy_JumpAerial5(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.85, true);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
    frame(fighter.lua_state_agent, 30.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, Hash40::new("k_bust"), 5, -5, 0, 0, 0, 0, 0.8, true);
    }
    frame(fighter.lua_state_agent, 33.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND_WORK(fighter, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_EFFECT_KIND_FLYING, false, true);
    }
}

//GlideStart
unsafe extern "C" fn effect_buddy_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
    LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.045, /*B*/ 0.1);
}

//GlideWing
unsafe extern "C" fn effect_buddy_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_buddy_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14.0, 0.0, -44.0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.4);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 14.4, -5.22, -44.0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 8.0, 0.0, 45.0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, 0.4);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 11.3, 9.9, 45.0, 0, 0, 1.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(fighter.lua_state_agent, 29.0);
    if is_excute(fighter) {
        EFFECT_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 12.4, 0.0, 25.0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, 0.4);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("buddy_attack_line"), Hash40::new("buddy_attack_line"), Hash40::new("top"), 0, 14.2, 1.4, 25.0, 0, 0, 1.3, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}  

//GlideLanding
unsafe extern "C" fn effect_buddy_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    Agent::new("buddy")
    .effect_acmd("effect_jumpaerialf3", effect_buddy_JumpAerialF3)
    .effect_acmd("effect_jumpaerialf4", effect_buddy_JumpAerialF4)
    .effect_acmd("effect_jumpaerialf5", effect_buddy_JumpAerialF5)
    .effect_acmd("effect_glidestart", effect_brave_GlideStart)
    .effect_acmd("effect_glidewing", effect_brave_GlideWing)
    .effect_acmd("effect_glideattack", effect_brave_GlideAttack)
    .effect_acmd("effect_glidelanding", effect_brave_GlideLanding)
    .install();
}
