use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//AttackLw3
    agent = "daisy", 
    script = "sound_attacklw3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_daisy_attacklw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE_REMAIN(fighter, Hash40::new("se_daisy_attackhard_l01"));
    }
}

#[acmd_script(//AttackAirN
    agent = "daisy", 
    script = "sound_attackairn", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_daisy_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_daisy_attackair_n01"));
    }
}

#[acmd_script(//AttackAirF
    agent = "daisy", 
    script = "sound_attackairf", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_daisy_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_daisy_attackair_f01"));
    }
}

#[acmd_script(//AttackAirLw
    agent = "daisy", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_daisy_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_daisy_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_daisy_attackair_l01"));
    }
}    

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_daisy_attacklw3,
        sound_daisy_attackairn,
        sound_daisy_attackairf,
        sound_daisy_attackairlw
    );
}