use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//BreathMove
    agent = "plizardon_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_firegfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0.0, 0.0, 0.0, 10.0, -3.0, 0.0, 1.0, 0.0, 2.0, 2.0, 10.0, 0.0, 0.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0, 10, -2, 90, 0, 180, 1.2, true);
    }
}

#[acmd_script(//GlideStart
    agent = "plizardon", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_glidestartgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 0, 0, 0, 0, 0, 6.0, true);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.35, /*B*/ 0.08);
    }
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("fire"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
    }
}

#[acmd_script(//GlideWing
    agent = "plizardon", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_glidegfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "plizardon", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_glideattackgfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("fire"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("mouth2"), 0, 0, 0, 0, 0, 0, 0.35, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("wingl1"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("wingr1"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
    }
}  

#[acmd_script(//GlideLanding
    agent = "plizardon", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_glidelandinggfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        plizardon_firegfx,
        plizardon_glidestartgfx,
        plizardon_glidegfx,
        plizardon_glideattackgfx,
        plizardon_glidelandinggfx
    );
}
