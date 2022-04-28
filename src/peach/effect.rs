use smash::app::sv_animcmd::*;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;
use smash_script::*;
use smashline::*;

#[acmd_script(
    agent = "peach",
    script = "effect_kamehameha_fire",
    category = ACMD_EFFECT,
low_priority )]
unsafe fn effect_peach_kamehameha_fire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_genesis_beam"), Hash40::new("top"), 10, 9, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 67.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_genesis_beam"), true, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_peach_kamehameha_fire,
    //    effect_peach_kamehameha_charge,
     //   effect_peach_kamehameha_start
    );
}