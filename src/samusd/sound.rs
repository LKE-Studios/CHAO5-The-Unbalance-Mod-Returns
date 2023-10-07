use crate::imports::BuildImports::*;

#[acmd_script(//AttackS3, AttackS3Lw, AttackS3Hi
    agent = "samusd", 
    scripts = ["sound_attacks3", "sound_attacks3hi", "sound_attacks3lw"], 
    category = ACMD_SOUND)]
unsafe fn sound_samusd_attacks3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_m"));
    }
    frame(fighter.lua_state_agent, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_special_n03"));
    }
}

#[acmd_script(//AttackLw4
    agent = "samusd", 
    script = "sound_attacklw4", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_samusd_attacklw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_common_smash_start"));
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
    }
}

#[acmd_script(//AttackAirF
    agent = "samusd", 
    script = "sound_attackairf", 
    category = ACMD_SOUND)]
unsafe fn sound_samusd_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

#[acmd_script(//AttackAirB
    agent = "samusd", 
    script = "sound_attackairb",
    category = ACMD_SOUND)]
unsafe fn sound_samusd_attackairb(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

#[acmd_script(//AttackAirHi
    agent = "samusd", 
    script = "sound_attackairhi", 
    category = ACMD_SOUND)]
unsafe fn sound_samusd_attackairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_swing_09"));
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

#[acmd_script(//AttackAirLw 
    agent = "samusd", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_samusd_attackairlw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_samusd_attacks3,
        sound_samusd_attackairf,
        sound_samusd_attacklw4,
        sound_samusd_attackairf,
        sound_samusd_attackairb,
        sound_samusd_attackairhi,
        sound_samusd_attackairlw
    );
}