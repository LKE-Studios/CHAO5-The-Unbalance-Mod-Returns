use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//Shot
    agent = "peach", 
    script = "effect_shot", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn peach_spore(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("peach_kinopio_bullet"), Hash40::new("top"), 0, 0, 3, 0, 0, 0, 3.2, true);
    }
} 

pub fn install() {
    smashline::install_acmd_scripts!(
        peach_spore
    );
}