use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//Attack13
    agent = "lucina", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucina_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
}

#[acmd_script(//AttackHi4
    agent = "lucina", 
    script = "sound_attackhi4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucina_attackhi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_common_smash_start_02"));
        macros::PLAY_SE(fighter, Hash40::new("vc_lucina_attack06"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_smash_h01"));
    }
}

#[acmd_script(//AttackAirN
    agent = "lucina", 
    script = "sound_attackairn", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucina_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

#[acmd_script(//AttackAirB
    agent = "lucina", 
    script = "sound_attackairb", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucina_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_lucina_attack13,
        sound_lucina_attackhi4,
        sound_lucina_attackairn,
        sound_lucina_attackairb
    );
}