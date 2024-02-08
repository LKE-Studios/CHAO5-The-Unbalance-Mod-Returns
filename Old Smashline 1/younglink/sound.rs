use crate::imports::BuildImports::*;

#[acmd_script(//AttackDash
    agent = "younglink", 
    script = "sound_attackdash", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_younglink_attackdash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l02"));
        PLAY_SE(fighter, Hash40::new("se_younglink_escape"))
    }
    wait(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_younglink_landing01"));
    }
}

#[acmd_script(//AttackAirF
    agent = "younglink", 
    script = "sound_attackairf", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_younglink_attackairf(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_younglink_rnd_attack"));
        PLAY_SE(fighter, Hash40::new("se_younglink_swing_l"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_younglink_attackdash,
        sound_younglink_attackairf
    );
}