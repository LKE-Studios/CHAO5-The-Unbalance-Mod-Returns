use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialAirHiGFX
    agent = "donkey", 
    script = "effect_specialairhi", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn donkey_upbairgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 9.0);
    for _ in 0..8 {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, true);
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("donkey_spin_wind"), Hash40::new("donkey_spin_wind"), Hash40::new("top"), 0, 10, -3, 0, 0, 5, 2.7, true, *EF_FLIP_YZ);
        }
        wait(fighter.lua_state_agent, 8.0);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        donkey_upbairgfx,
    );
}