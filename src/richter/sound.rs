use smash::app::sv_animcmd::*;
use smash::app::lua_bind::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::{*, macros::*};
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//Fly
    agent = "richter_axe", 
    script = "sound_fly", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn richter_axesfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_richter_special_n02"), 0.75);
        PLAY_SE(fighter, Hash40::new("se_richter_special_n02"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        richter_axesfx
    );
}