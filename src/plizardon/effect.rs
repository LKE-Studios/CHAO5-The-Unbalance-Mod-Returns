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
        lua_args!(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0.0, 0.0, 0.0, 10.0, -3.0, 90.0, 1.0, 180.0, 2.0, 2.0, 10.0, 0.0, 0.0, true);
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
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("plizardon_atk_fire_air"), Hash40::new("fire"), 0, 0, 0, 0, 0, 0, 1, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4)
    }
    frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
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
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4)
    }
    frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("plizardon_atk_fire_air"), false, false);
    }
}  

pub fn install() {
    smashline::install_acmd_scripts!(
        plizardon_firegfx,
        plizardon_glidestartgfx,
        plizardon_glideattackgfx
    );
}
