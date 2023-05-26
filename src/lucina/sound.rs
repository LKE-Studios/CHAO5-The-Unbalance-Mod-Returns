use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::phx::Vector3f;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::L2CAgentBase;
use smashline::*;
use smash_script::*;

#[acmd_script(//AttackAirN
    agent = "lucina", 
    script = "sound_attackairn", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_lucina_attackairn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_lucina_rnd_attack"));
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_l"));
    }
    frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_lucina_swing_ll"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_lucina_attackairn
    );
}