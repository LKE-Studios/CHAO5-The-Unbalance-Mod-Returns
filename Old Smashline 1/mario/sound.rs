use crate::imports::BuildImports::*;
use crate::mario::frame::*;

#[acmd_script(//SpecialN, SpecialAirN
    agent = "mario", 
    scripts = ["sound_specialn", "sound_specialairn"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn game_mario_specialn(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if MARIO_GIANT_FIREBALL[ENTRY_ID] == true {
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.7);
        }
    }
    else { 
        frame(fighter.lua_state_agent, 13.0);
        if is_excute(fighter) {
            PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        game_mario_specialn
    );
}