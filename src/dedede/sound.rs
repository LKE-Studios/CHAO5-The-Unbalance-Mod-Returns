use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//JumpAerialF3, JumpAerialF4, JumpAerialF5, JumpAerialF6, JumpAerialF7, JumpAerialF8, JumpAerialF9
    agent = "dedede", 
    scripts = ["sound_jumpaerialf3", "sound_jumpaerialf4", "sound_jumpaerialf5", 
    "sound_jumpaerialf6", "sound_jumpaerialf7", "sound_jumpaerialf8", "sound_jumpaerialf9"],
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_dedede_jumpaerial(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_dedede_jump02_05"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_dedede_jumpaerial,
    );
}