use crate::imports::BuildImports::*;

#[acmd_script(//Attack13
    agent = "lucas", 
    script = "sound_attack13", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attack13(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_swing_l"));
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
}

#[acmd_script(//AttackS4Charge
    agent = "lucas", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attacks4charge(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack003"));
    }
}

#[acmd_script(//AttackLw4
    agent = "lucas", 
    script = "sound_attacklw4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start_04"));
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l04"));
    }
    wait(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_attack07"));
    }
    wait(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l02"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_smash_l03"));
    }
}

#[acmd_script(//AttackAirLw
    agent = "lucas", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l05"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l01"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l02"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l03"));
    }
    wait(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_lucas_attackair_l04"));
    }
}

#[acmd_script(//AppealSL
    agent = "lucas", 
    script = "sound_appealsl", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsl(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//AppealSR
    agent = "lucas", 
    script = "sound_appealsr", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_appealsr(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_lucas_appeal03"));
    }
    frame(fighter.lua_state_agent, 26.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_lucas_win02"));
    }
}

#[acmd_script(//WessDance
    agent = "lucas", 
    script = "sound_wessdance", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucas_wessdance(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_lucas_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 1370.0);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        PLAY_SE_REMAIN(fighter, Hash40::new("se_audience_cheer_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_lucas_attack13,
        sound_lucas_attacks4charge,
        sound_lucas_attacklw4,
        sound_lucas_attackairlw,
        sound_lucas_appealsr,
        sound_lucas_appealsl,
        sound_lucas_wessdance
    );
}