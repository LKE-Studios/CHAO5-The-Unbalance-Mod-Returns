use crate::imports::BuildImports::*;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, 
    agent = "buddy", 
    scripts = ["effect_jumpaerialf3", "effect_jumpaerialf4", "effect_jumpaerialf5"],
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_buddy_jumpaerial(fighter: &mut L2CAgentBase) {
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

#[acmd_script(//GlideStart
    agent = "buddy", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_buddy_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.2, 0, 0, 0, 0, 3.0, true);
    LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 0.045, /*B*/ 0.1);
}

#[acmd_script(//GlideWingGFX
    agent = "buddy", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_buddy_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "buddy", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_buddy_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        EFFECT_FOLLOW(fighter, Hash40::new("buddy_special_s_wind"), Hash40::new("top"), 2.0, 4.0, 0, 0, 0, 0, 0.8, true);
    }
}  

#[acmd_script(//GlideLanding
    agent = "buddy", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_buddy_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_buddy_jumpaerial,
        effect_buddy_glidestart,
        effect_buddy_glidewing,
        effect_buddy_glideattack,
        effect_buddy_glidelanding
    );
}
