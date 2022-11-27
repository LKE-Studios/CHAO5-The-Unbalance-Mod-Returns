//use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
//use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//BurstAttack
    agent = "samusd_bomb", 
    script = "effect_burstattack", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn samusd_bombgfx(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("sys_bomb_a"), Hash40::new("sys_bomb_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        smash::app::sv_animcmd::EFFECT_BRANCH_SITUATION(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 0.12, /*G*/ 0.1, /*B*/ 1.85);
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
       samusd_bombgfx
    );
}