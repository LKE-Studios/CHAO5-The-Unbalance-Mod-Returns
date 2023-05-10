use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;

#[acmd_script(//AttackLw3
    agent = "koopa", 
    script = "sound_attacklw3", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopa_attacklw3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
        }
    } else {//Bowser
        frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l01"));
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_koopa_attackhard_l02"));
        }
    }
}

#[acmd_script(//AttackAirLw 
    agent = "koopa", 
    script = "sound_attackairlw", 
    category = ACMD_SOUND, 
    low_priority )]
unsafe fn sound_koopa_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_koopa_nailswing02"));
            macros::PLAY_SE(fighter, Hash40::new("vc_koopa_attack06"));
        }
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_koopa_smash_h01"));
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::PLAY_STATUS(fighter, Hash40::new("se_koopa_attackair_l01"));
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        sound_koopa_attacklw3,
        sound_koopa_attackairlw
    );
}