use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
//use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialSF, SpecialAirSF
    agent = "master", 
    scripts = ["effect_specialsf", "effect_specialairsf"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn master_sidebfgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_spear_aura"), false, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, false);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_slash_air"), Hash40::new("top"), 2.0, 13.0, 8.0, 8.0, -5.0, 68.0, 2.0, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_slash_particle_end"), Hash40::new("top"), 2.0, 13.0, 8.0, 8.0, -5.0, 68.0, 2.0, true);
        EffectModule::set_disable_render_offset_last(fighter.module_accessor);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_spear_slash_particle"), Hash40::new("top"), 2.0, 15.5, 8.0, 8.0, -5.0, 68.0, 2.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8.0, 6.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("master_spear_slash_air"), -1)
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_spear_slash_particle"), false, true)
    }
}

#[acmd_script(//SpecialSFDash, SpecialAirSFDash
    agent = "master", 
    scripts = ["effect_specialsfdash", "effect_specialairsfdash"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn master_sidebfdashgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_spear_aura"), false, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_aura"), Hash40::new("haver"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("master_spear_scrape"), Hash40::new("top"), -16, 0, 8, 0, 18, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("master_spear_scrape_ground"), Hash40::new("top"), -6, 0, -15, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("master_spear_scrape_dash"), Hash40::new("top"), -6, 0, -15, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_slash"), Hash40::new("top"), 2.0, 15.0, 8.0, 8.0, -5.0, 68.0, 2.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("master_spear_slash_particle_end"), Hash40::new("top"), 2.0, 15.0, 8.0, 8.0, -5.0, 68.0, 1.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("master_spear_slash_particle"), Hash40::new("top"), 2.0, 14.0, 8.0, 8.0, -5.0, 68.0, 1.7, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT(fighter, Hash40::new("master_spear_scrape_ground"), Hash40::new("top"), -6.0, 0, -10.0, 0, 0, 0, 1.6, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("master_spear_scrape_dash"), Hash40::new("top"), -6.0, 0, -10.0, 0, 0, 0, 1.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        macros::EFFECT(fighter, Hash40::new("master_spear_scrape_ground"), Hash40::new("top"), -6.0, 0, 0, 0, 0, 0, 1.8, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("master_spear_scrape_dash"), Hash40::new("top"), -6.0, 0, 0, 0, 0, 0, 1.6, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("master_spear_scrape_ground"), Hash40::new("top"), -6.0, 0.0, 5.0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT_FLW_UNSYNC_VIS(fighter, Hash40::new("master_spear_scrape_dash"), Hash40::new("top"), -6.0, 0.0, 5.0, 0.0, 0.0, 0.0, 2.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FLW_POS(fighter, Hash40::new("master_spear_aura_particle"), Hash40::new("haver"), -8.0, 6.0, 0.0, 0.0, 0.0, 0.0, 1.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_spear_scrape"), false, true);
    }
    frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("master_spear_slash_particle"), false, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        master_sidebfgfx,
        master_sidebfdashgfx
    );
}