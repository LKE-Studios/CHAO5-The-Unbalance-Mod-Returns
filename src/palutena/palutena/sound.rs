use crate::imports::BuildImports::*;

//ShieldBreakFly
unsafe extern "C" fn sound_palutena_ShieldBreakFly(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_palutena_rnd_futtobi01"));
    }
}

//GlideStart
unsafe extern "C" fn sound_palutena_GlideStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_dash_start"));
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_jump02"));
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide")); //75
        PLAY_SE_REMAIN(fighter, Hash40::new("se_palutena_glide_loop")); //76
    }
}

//GlideAttack
unsafe extern "C" fn sound_palutena_GlideAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_palutena_special_l03"));
    }
}

//GlideLanding
unsafe extern "C" fn sound_palutena_GlideLanding(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_LANDING_SE(fighter, Hash40::new("se_palutena_landing02"));
    }
}

//GlideEnd
unsafe extern "C" fn sound_palutena_GlideEnd(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_palutena_glide_loop"));
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_palutena_escapeair"));
        PLAY_SE(fighter, Hash40::new("se_palutena_jump01"));
    }
}   

//AttackHi4
unsafe extern "C" fn sound_palutena_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_03"));
        PLAY_SE(fighter, Hash40::new("vc_palutena_attack07"));
        PLAY_SE(fighter, Hash40::new("se_palutena_smash_h01"));
    }
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLAG_ATTACK_HI4_IS_CHARGED) {
        frame(fighter.lua_state_agent, 23.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_h01"));
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_palutena_smash_h01"));
        }
    }
}

pub fn install() {
    Agent::new("palutena")
    .sound_acmd("sound_shieldbreakfly", sound_palutena_ShieldBreakFly)
    .sound_acmd("sound_glidestart", sound_palutena_GlideStart)
    .sound_acmd("sound_glideattack", sound_palutena_GlideAttack)
    .sound_acmd("sound_glidelanding", sound_palutena_GlideLanding)
    .sound_acmd("sound_glideend", sound_palutena_GlideEnd)
    .sound_acmd("sound_attackhi4", sound_palutena_AttackHi4)
    .install();
}