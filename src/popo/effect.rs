//use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
//use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//Fly
    agent = "popo_blizzard", 
    script = "effect_fly", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn popo_blizzardgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("popo_blizzerd_bullet"), Hash40::new("top"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 3.5, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        popo_blizzardgfx
    );
}