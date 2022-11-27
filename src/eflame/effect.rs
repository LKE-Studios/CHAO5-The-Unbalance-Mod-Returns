use smash::phx::Hash40;
//use smash::lib::lua_const::*;
//use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//SpecialHi
    agent = "eflame_firepillar", 
    script = "effect_specialhi",
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn eflame_pillargfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_ground"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.3, 0, 0, 0, 0, 0, 0, true);
        macros::EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        macros::EFFECT(fighter, Hash40::new("eflame_promrevolt_firepillar_impact"), Hash40::new("top"), 0, 1, 0, 0, 0, 0, 3.5, 0, 0, 0, 0, 0, 0, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        eflame_pillargfx
    );
}