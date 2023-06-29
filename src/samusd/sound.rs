use crate::imports::BuildImports::*;

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
        sound_samusd_attacklw4,
        sound_samusd_attackairlw
    );
}