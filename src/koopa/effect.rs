use smash::app::sv_animcmd::*;
use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CAgentBase;
use smash::app::*;
use smash::app::lua_bind::*;

#[acmd_script(//AttackLw3 
    agent = "koopa", 
    script = "effect_attacklw3", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_attacklw3(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 6, 6, 7, -34, -20, 1.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.4);
        }    
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 6, 6, 7, -34, -20, 1.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 1.5);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FLIP(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 6, 8, -4, -59, -167, 1.85, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_RATE(fighter, 2.3);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

#[acmd_script(//AttackAirLw 
    agent = "koopa", 
    script = "effect_attackairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_attackairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 16.0);
        if macros::is_excute(fighter) {
			macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
		}
    } else { //Bowser
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, -5, -5, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, true);
        }
        frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("koopa_atk_fall"), Hash40::new("top"), 0, 2.5, 0, 0, 0, 0, 0.65, true, 1, 1, 1);
            smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
            fighter.pop_lua_stack(1);
        }
    }
}

#[acmd_script(//SpecialSCatch
    agent = "koopa", 
    script = "effect_specialscatch", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_specialscatch(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("koopa_drop"), Hash40::new("top"), 0, 12, 0, -90, 0, 90, 1, false, 0.941, 1, 0.863);
            smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 0, 11, 8, 5, -3, -4, 2.0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.17, /*B*/ 1.06);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::FLASH(fighter, 1, 1, 0.753, 0.627);
            macros::FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_drop_arc"), true, true);
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 2.0);
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) <= 0.0 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 2.5, -90, -204, 1.8, true, 0.1);
            }
            else {
                if macros::is_excute(fighter) {
                    macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 18.7, -90, 2, 1.8, true, 0.1);
                }
            }
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::FLASH(fighter, 1, 1, 0.753, 0.627);
            macros::FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
    }
}

#[acmd_script(//SpecialSAirCatch
    agent = "koopa", 
    script = "effect_specialsaircatch", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_specialsaircatch(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("koopa_drop"), Hash40::new("top"), 0, 12, 0, -90, 0, 90, 1, false, 0.941, 1, 0.863);
            smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
        frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("koopa_scratch"), Hash40::new("koopa_scratch"), Hash40::new("top"), 0, 11, 8, 5, -3, -4, 2.0, true, *EF_FLIP_YZ);
            macros::LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.8, /*G*/ 0.17, /*B*/ 1.06);
        }
        frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::FLASH(fighter, 1, 1, 0.753, 0.627);
            macros::FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_drop_arc"), true, true);
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("koopa_atk_arc"), Hash40::new("koopa_atk_arc"), Hash40::new("top"), 0, 12.5, 3, 18.7, -90, 2, 1.8, true, *EF_FLIP_YZ, 0.1);
        }
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 1, 1, 0.753, 0.627);
            macros::FLASH_FRM(fighter, 5, 0.502, 0, 0, 0);
        }
        wait(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::COL_NORMAL(fighter);
        }
    }
}

#[acmd_script(//SpecialLw 
    agent = "koopa", 
    script = "effect_speciallw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_speciallw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 0, 0);
            macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
            macros::COL_NORMAL(fighter);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        }
        for _ in 0..6 {
            if macros::is_excute(fighter) {
                macros::FLASH(fighter, 0, 0, 0, 0);
                macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
                macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
                macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::BURN_COLOR_NORMAL(fighter);
                macros::COL_NORMAL(fighter);
            }
            wait(fighter.lua_state_agent, 1.0);
        }
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 0, 0);
            macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
            macros::COL_NORMAL(fighter);
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 9.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_FLIP_ALPHA(fighter, Hash40::new("koopa_drop_arc"), Hash40::new("koopa_drop_arc"), Hash40::new("top"), -7, 15, 8, 12.46, -45.812, 61.035, 1.15, true, *EF_FLIP_YZ, 0.7);
        }
        frame(fighter.lua_state_agent, 13.0);
        if macros::is_excute(fighter) {
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("koopa_drop_arc"), true, true);
        }
        frame(fighter.lua_state_agent, 30.0);
        if macros::is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("koopa_drop"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, false, 0.941, 1, 0.863);
            smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    }
}

#[acmd_script(//SpecialAirLw 
    agent = "koopa", 
    script = "effect_specialairlw", 
    category = ACMD_EFFECT, 
    low_priority )]
unsafe fn effect_koopa_specialairlw(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) >= 8 && 
    WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) <= 15 { //Midbus
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 0, 0);
            macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
            macros::COL_NORMAL(fighter);
        }
        frame(fighter.lua_state_agent, 32.0);
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 2.0, true, 2);
            macros::FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), -2.0, 0, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
        }
        for _ in 0..6 {
            if macros::is_excute(fighter) {
                macros::FLASH(fighter, 0, 0, 0, 0);
                macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
                macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
                macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
            }
            wait(fighter.lua_state_agent, 2.0);
            if macros::is_excute(fighter) {
                macros::BURN_COLOR_NORMAL(fighter);
                macros::COL_NORMAL(fighter);
            }
            wait(fighter.lua_state_agent, 1.0);
        }
        if macros::is_excute(fighter) {
            macros::FLASH(fighter, 0, 0, 0, 0);
            macros::BURN_COLOR(fighter, 2, 2, 0.5, 0.9);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0.7);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::FLASH_FRM(fighter, 2, 0, 0, 0, 0);
            macros::BURN_COLOR_FRAME(fighter, 2, 2, 2, 0.5, 0);
        }
        wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::BURN_COLOR_NORMAL(fighter);
            macros::COL_NORMAL(fighter);
        }
    } else { //Bowser
        frame(fighter.lua_state_agent, 29.0);
        if macros::is_excute(fighter) {
            fighter.clear_lua_stack();
            lua_args!(fighter, Hash40::new("koopa_drop_air"), Hash40::new("top"), 0, 12, 0, 0, 0, 0, 1, true, 0.941, 1, 0.863);
            smash::app::sv_animcmd::EFFECT_FOLLOW_COLOR(fighter.lua_state_agent);
        }
    }
}

pub fn install() {
    smashline::install_acmd_scripts!(
        effect_koopa_attacklw3,
        effect_koopa_attackairlw,
        effect_koopa_specialscatch,
        effect_koopa_specialsaircatch,
        effect_koopa_speciallw,
        effect_koopa_specialairlw
    );
}