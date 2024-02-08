use crate::imports::BuildImports::*;

#[acmd_script(//AttachWall
    agent = "link", 
    script = "sound_attachwall", 
    category = ACMD_SOUND)]
unsafe fn sound_link_attachwall(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        STOP_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        STOP_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
    }
}

#[acmd_script(//AttachWallClimb
    agent = "link", 
    script = "sound_attachwallclimb", 
    category = ACMD_SOUND)]
unsafe fn sound_link_attachwallclimb(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_link_step_right_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_link_step_left_s_ft"));
        }
        wait(fighter.lua_state_agent, 5.0);
    }
}

pub fn install() {
    install_acmd_scripts!(
        sound_link_attachwall,
        sound_link_attachwallclimb
    );
}