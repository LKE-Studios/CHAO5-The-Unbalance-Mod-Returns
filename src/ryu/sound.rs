use smash::app::sv_animcmd::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_start",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_start(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n01"));
    }
}

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_charge",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_charge(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n02"));
    }
}

#[acmd_script(
agent = "ryu",
script = "sound_kamehameha_fire",
category = ACMD_SOUND,
low_priority )]
unsafe fn sound_ryu_kamehameha_fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_ryu_special_n03"));
    }
    frame(fighter.lua_state_agent, 67.0);
    if macros::is_excute(fighter) {
        macros::STOP_SE(fighter, Hash40::new("se_ryu_special_n03"));
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_ryu_kamehameha_start,
        sound_ryu_kamehameha_charge,
        sound_ryu_kamehameha_fire
    );
}