use crate::imports::BuildImports::*;

#[acmd_script(//SpecialAirHi
    agent = "donkey", 
    script = "sound_specialairhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_donkey_specialairhi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        PLAY_STATUS(fighter, Hash40::new("se_donkey_special_h02"));
    }
    wait(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h04"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h05"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h06"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
    wait(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_donkey_special_h07"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_donkey_specialairhi,
    );
}