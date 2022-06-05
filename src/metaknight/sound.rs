use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//ThrowHiSFX
    agent = "metaknight", 
    script = "sound_throwhi", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn metaknight_throwupsfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_02"));
    }
    wait(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_throw_03"));
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_metaknight_rnd_attack"));
    }
    wait(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::PLAY_DOWN_SE(fighter, Hash40::new("se_common_down_soil_s"));
        macros::PLAY_SE(fighter, Hash40::new("se_common_kick_hit_m"));
        macros::PLAY_SE(fighter, Hash40::new("se_metaknight_final_hit"));
    }
}     

pub fn install() {
    smashline::install_acmd_scripts!(
        metaknight_throwupsfx
    );
}