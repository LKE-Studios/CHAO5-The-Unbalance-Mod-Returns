use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//GlideStart
    agent = "pit", 
    script = "sound_glidestart", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn pit_glide1sfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_jump02"));
    }
    frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pit_special_h01"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        pit_glide1sfx
    );
}