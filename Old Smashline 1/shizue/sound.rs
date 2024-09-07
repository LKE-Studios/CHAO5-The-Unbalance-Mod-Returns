use crate::imports::BuildImports::*;

#[acmd_script(//AttackS4ChargeSFX
    agent = "shizue", 
    script = "sound_attacks4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn shizue_sidesmashsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

#[acmd_script(//AttackHi4ChargeSFX
    agent = "shizue", 
    script = "sound_attackhi4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn shizue_upsmashsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

#[acmd_script(//AttackLw4ChargeSFX
    agent = "shizue", 
    script = "sound_attacklw4charge", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn shizue_downsmashsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_smash_start_04"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        shizue_sidesmashsfx,
        shizue_upsmashsfx,
        shizue_downsmashsfx
    );
}