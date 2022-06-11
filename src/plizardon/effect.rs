use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;

#[acmd_script(//BreathMove
    agent = "plizardon_breath", 
    script = "effect_move", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn plizardon_firegfx(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        fighter.clear_lua_stack();
        lua_args!(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0.0, 0.0, 0.0, 10.0, -3.0, 90.0, 1.0, 180.0, 2.0, 2.0, 10.0, 0.0, 0.0, true);
        smash::app::sv_animcmd::EFFECT_FOLLOW_RND(fighter.lua_state_agent);
        fighter.pop_lua_stack(1);
        //macros::EFFECT_FOLLOW(fighter, Hash40::new("plizardon_kaenhousya"), Hash40::new("top"), 0, 10, -2, 90, 0, 180, 1.2, true);
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        plizardon_firegfx
    );
}
