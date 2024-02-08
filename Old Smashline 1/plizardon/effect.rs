use crate::imports::BuildImports::*;

#[acmd_script(//BreathMove
    agent = "plizardon_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_breath_move(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0.0, 0.0, 0.0, 10.0, -3.0, 0.0, 1.0, 0.0, 2.0, 2.0, 10.0, 0.0, 0.0, true);
        EFFECT_FOLLOW_RND(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
    }
}

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_glidestart(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//GlideWing
    agent = "plizardon", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "plizardon", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_glideattack(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//GlideLanding
    agent = "plizardon", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}

#[acmd_script(//SpecialAirHi2Start
    agent = "plizardon", 
    script = "effect_specialairhi2start", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_specialairhi2start(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//SpecialAirHi2
    agent = "plizardon", 
    script = "effect_specialairhi2", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_specialairhi2(fighter: &mut L2CAgentBase) {
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


#[acmd_script(//SpecialAirHi2Landing
    agent = "plizardon", 
    script = "effect_specialairhi2landing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_plizardon_specialairhi2landing(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("plizardon_atk_lw4"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_crown_collision"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_ground_shockwave"), Hash40::new("top"), 1, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        EFFECT(fighter, Hash40::new("sys_bomb_d"), Hash40::new("top"), 0.0, 0.0, 0.0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}


pub fn install() {
    smashline::install_acmd_scripts!(
        effect_plizardon_breath_move,
        effect_plizardon_glidestart,
        effect_plizardon_glidewing,
        effect_plizardon_glideattack,
        effect_plizardon_glidelanding,
        effect_plizardon_specialairhi2start,
        effect_plizardon_specialairhi2,
        effect_plizardon_specialairhi2landing
    );
}
