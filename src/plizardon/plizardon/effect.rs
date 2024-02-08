use crate::imports::BuildImports::*;

//GlideStart
unsafe extern "C" fn effect_plizardon_GlideStart(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 5.2, 0, 0, 0, 0, 3.4, true);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.35, /*B*/ 0.08);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("fire"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
    }
}

//GlideWing
unsafe extern "C" fn effect_plizardon_GlideWing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

//GlideAttack
unsafe extern "C" fn effect_plizardon_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("fire"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("mouth2"), 0, 0, 0, 0, 0, 0, 0.35, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("wingl1"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("wingr1"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
    }
}  

//GlideLanding
unsafe extern "C" fn effect_plizardon_GlideLanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirHi2Start
unsafe extern "C" fn effect_plizardon_SpecialAirHi2Start(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("plizardon_flare_blitz_hold"), Hash40::new("rot"), 0, 1, 0, 0, 0, 0, 0.8, true);
    }
    BURN_COLOR_NORMAL(fighter);
    if is_excute(fighter) {
        FLASH(fighter, 1, 0.5, 0.0, 0.6);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        FLASH_FRM(fighter, 2, 1, 0.3, 0.1, 0.7);
    }
    wait(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        COL_NORMAL(fighter);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR(fighter, 2, 0.1, 0, 0.4);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 2, 0.5, 0.2, 0.1, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
}

//SpecialAirHi2
unsafe extern "C" fn effect_plizardon_SpecialAirHi2(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("plizardon_flare_blitz"), Hash40::new("plizardon_flare_blitz"), Hash40::new("rot"), 0.0, -15.0, 6.0, 160.0, 0.0, 0, 1.15, true, *EF_FLIP_NONE);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        BURN_COLOR(fighter, 2, 0.1, 0, 0.7);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR_FRAME(fighter, 2, 1, 0.2, 0.1, 0);
    }
    wait(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        BURN_COLOR_NORMAL(fighter);
    }
    wait(fighter.lua_state_agent, 1.0);
}


//SpecialAirHi2Landing
unsafe extern "C" fn effect_plizardon_SpecialAirHi2Landing(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("plizardon_atk_lw4"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_crown_collision"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}

pub unsafe extern "C" fn effect_plizardon_SpecialZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter,0.5);
        LAST_EFFECT_SET_COLOR(fighter,0.5,0.5,0.5);
    }
}

pub unsafe extern "C" fn effect_plizardon_SpecialAirZ(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_misfire"), Hash40::new("top"), 11, 9, 0, 0, 0, 0, 3.0, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter,0.5);
        LAST_EFFECT_SET_COLOR(fighter,0.5,0.5,0.5);
    }
}

pub fn install() {
    Agent::new("plizardon")
    .effect_acmd("effect_glidestart", effect_plizardon_GlideStart)
    .effect_acmd("effect_glidewing", effect_plizardon_GlideWing)
    .effect_acmd("effect_glideattack", effect_plizardon_GlideAttack)
    .effect_acmd("effect_glidelanding", effect_plizardon_GlideLanding)
    .effect_acmd("effect_specialairhi2start", effect_plizardon_SpecialAirHi2Start)
    .effect_acmd("effect_specialairhi2", effect_plizardon_SpecialAirHi2)
    .effect_acmd("effect_specialairhi2landing", effect_plizardon_SpecialAirHi2Landing)
    .effect_acmd("effect_specialz", effect_plizardon_SpecialZ)
    .effect_acmd("effect_specialairz", effect_plizardon_SpecialAirZ)
    .install();
}
