use crate::imports::BuildImports::*;

#[acmd_script(//GlideStart
    agent = "ridley", 
    script = "effect_glidestart", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_ridley_glidestart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -4.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, true);
    EFFECT_FOLLOW(fighter, Hash40::new("sys_aura_light"), Hash40::new("top"), 0.0, 4.8, 0, 0, 0, 0, 3.5, true);
    LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.39, /*G*/ 0.045, /*B*/ 1.55);
}

#[acmd_script(//GlideWing
    agent = "ridley", 
    script = "effect_glidewing", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_ridley_glidewing(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
    }
}

#[acmd_script(//GlideAttack
    agent = "ridley", 
    script = "effect_glideattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_ridley_glideattack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ridley_grabbing_hold"), Hash40::new("havel"), -1.0, 0, 0, 0, 0, 0, 1.0, true);
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ridley_scratch"), Hash40::new("ridley_scratch"), Hash40::new("top"), -5.0, 8.0, 8.0, -124.0, 72.5, 70, 1, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("ridley_scratch"), Hash40::new("ridley_scratch"), Hash40::new("top"), 0, 17, 4, -24.6, -5, 209, 2.0, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.5, /*G*/ 0.045, /*B*/ 0.65);
    }
}  

#[acmd_script(//GlideLanding
    agent = "ridley", 
    script = "effect_glidelanding", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_ridley_glidelanding(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.48, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_ridley_glidestart,
        effect_ridley_glidewing,
        effect_ridley_glideattack,
        effect_ridley_glidelanding
    );
}
