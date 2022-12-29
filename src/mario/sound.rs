use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;
use crate::mario::frame::*;

#[acmd_script(//SpecialN, SpecialAirN
    agent = "mario", 
    script = "sound_specialn", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn mario_neutralbsfx(fighter: &mut L2CAgentBase) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if MARIO_GIANT_FIREBALL[ENTRY_ID] == true {
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
            SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_mario_special_n01"), 0.7);
        }
    }
    else { 
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_mario_special_n01"));
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        mario_neutralbsfx
    );
}