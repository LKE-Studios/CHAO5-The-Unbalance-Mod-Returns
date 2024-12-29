use crate::imports::BuildImports::*;

//JumpAerialF
unsafe extern "C" fn effect_kamek_JumpAerialFront(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 0, 0, 90, 0, 90, 1.85, 0, 0, 0, 0, 360, 0, true);
    }
}

//JumpAerialB
unsafe extern "C" fn effect_kamek_JumpAerialBack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_jump_aerial"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("ness_psi_rush"), Hash40::new("top"), 0, 0, 0, 90, 0, 90, 1.85, 0, 0, 0, 0, 360, 0, true);
    }
}

unsafe extern "C" fn effect_kamek_Run(fighter: &mut L2CAgentBase) {
    loop {
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 12.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 18.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 24.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 30.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 36.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 42.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
        }
        wait(fighter.lua_state_agent, 6.0);
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false);
    }
}

//EntryL
unsafe extern "C" fn effect_kamek_EntryL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("hip"), -4, 0, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("hip"), 4, 0, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flash"), false, false);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
}

//EntryR
unsafe extern "C" fn effect_kamek_EntryR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("hip"), -4, 0, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("hip"), 4, 0, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flash"), false, false);
    }
    frame(fighter.lua_state_agent, 41.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
}

//Attack11 
unsafe extern "C" fn effect_kamek_Attack11(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0.0, 12.0, 8.0, 90, 0, 0, 1.0, true);
        LAST_PARTICLE_SET_COLOR(fighter, 1.0, 0.6, 0.0);
        LAST_EFFECT_SET_ALPHA(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_speedline"), false, true);
    }
}

//AttackDash
unsafe extern "C" fn effect_kamek_AttackDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 4.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flash"), false, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_flash"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 7, 5, 180, 145, -25, 0.9, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.5, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackS3
unsafe extern "C" fn effect_kamek_AttackS3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
        EFFECT_ALPHA(fighter, Hash40::new("rosetta_tico_dead"), Hash40::new("top"), 0, 4.0, 19.2, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true, 1.0);
    }
}

//AttackHi3
unsafe extern "C" fn effect_kamek_AttackHi3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 11, 3.5, 0, 95, 90, 1.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_RATE(fighter, 0.8);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackLw3
unsafe extern "C" fn effect_kamek_AttackLw3(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackS4
unsafe extern "C" fn effect_kamek_AttackS4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kamek_broom_a"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("throw"), 3.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("throw"), 3.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("throw"), 3.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("throw"), 3.0, 0, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("throw"), 3.0, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

//AttackHi4
unsafe extern "C" fn effect_kamek_AttackHi4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kamek_spike"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.0, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("kamek_box"), 0, 1, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 104.0/255.0, 80.0/255.0, 0.0/255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT(fighter, Hash40::new("sys_attack_impact"), Hash40::new("kamek_box"), 0, 1, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 213.0/255.0, 0.0/255.0);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(fighter.lua_state_agent, 46.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_catch"), Hash40::new("kamek_box"), 0, 0, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 48.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackLw4
unsafe extern "C" fn effect_kamek_AttackLw4(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 9, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_shield_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_shield_smoke"), false, true);
    }
    frame(fighter.lua_state_agent, 37.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackAirN
unsafe extern "C" fn effect_kamek_AttackAirN(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -0.5, 0, 180, 0, -17.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, -0.5, 0, 180, 90, -17.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
}

//AttackAirF 
unsafe extern "C" fn effect_kamek_AttackAirF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.94, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackAirB
unsafe extern "C" fn effect_kamek_AttackAirB(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), -6, 6.5, -1, 0, 180, 270, 1.2, true, *EF_FLIP_YZ);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.8, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.2, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.0, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackAirHi
unsafe extern "C" fn effect_kamek_AttackAirHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 6.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 68.0/255.0, 0.0/255.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.9, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 68.0/255.0, 0.0/255.0);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 22.5, 0.5, 0, 0, 0, 0.7, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bomb_a"), Hash40::new("top"), 0, 25.5, 0.5, 0, 0, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//AttackAirLw
unsafe extern "C" fn effect_kamek_AttackAirLw(fighter: &mut L2CAgentBase) {}

//Catch
unsafe extern "C" fn effect_kamek_Catch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_counteract_mark"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(fighter, 136.0/255.0, 0.0/255.0, 226.0/255.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//CatchTurn
unsafe extern "C" fn effect_kamek_CatchTurn(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_counteract_mark"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(fighter, 136.0/255.0, 0.0/255.0, 226.0/255.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//CatchDash
unsafe extern "C" fn effect_kamek_CatchDash(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_counteract_mark"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.75, true);
        LAST_EFFECT_SET_COLOR(fighter, 136.0/255.0, 0.0/255.0, 226.0/255.0);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
}

//CatchAttack
unsafe extern "C" fn effect_kamek_CatchAttack(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 90, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 180, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 270, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
    }
}

//CatchWait
unsafe extern "C" fn effect_kamek_CatchWait(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("ness_psi_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 90, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 180, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 270, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
    }
}

//CatchPull
unsafe extern "C" fn effect_kamek_CatchPull(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 0, 0, 90, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 90, 0, 180, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 180, 0, 270, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_falling_smoke"), Hash40::new("throw"), 1.0, 0, 3.0, 270, 0, 0, 2.0, true);
        LAST_EFFECT_SET_COLOR(fighter, 188.0/255.0, 0.0/255.0, 255.0/255.0);
        LAST_EFFECT_SET_ALPHA(fighter, 2.0);
    }
}

//ThrowF
unsafe extern "C" fn effect_kamek_ThrowF(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 8.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 1.35, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 40, 0, 0, 0, 0, 0, 1.45, 0, 0, 0, 0, 0, 0, false);
    }
}

//ThrowB 
unsafe extern "C" fn effect_kamek_ThrowB(fighter: &mut L2CAgentBase) {
    if PostureModule::lr(fighter.module_accessor) == -1.0 {
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10.0, 0, -10, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, -10, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 20.0, 0, -10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
    else {
        frame(fighter.lua_state_agent, 4.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 10.0, 0, 10, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 6.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, 0, 10, 0, 180, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
        }
        frame(fighter.lua_state_agent, 8.0);
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 20.0, 0, 10, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

//ThrowHi
unsafe extern "C" fn effect_kamek_ThrowHi(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 7.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        if get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_l"), Hash40::new("kamek_spike"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("mario_fb_bullet_r"), Hash40::new("kamek_spike"), 0, 0, 0, 0, 0, 0, 1, true);
        }
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -3.2, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_l"), false, false);
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_bullet_r"), false, false);
    }
}      

//ThrowLw
unsafe extern "C" fn effect_kamek_ThrowLw(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("kamek_spike"), 0, 4, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 27.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_crown"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 20, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffCatch
unsafe extern "C" fn effect_kamek_CliffCatch(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_cliff_catch"), Hash40::new("havel"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

//CliffAttack
unsafe extern "C" fn effect_kamek_CliffAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 14.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("toer"), 4, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -2, 0, -2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 4, 1, -5, 25, -170, 0.85, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.8);
    }
}

//DownAttackU
unsafe extern "C" fn effect_kamek_DownAttackU(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 4.5, -6, 0, 120, 13, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 4.5, 5, 0, -50, -10, 1, true, *EF_FLIP_YZ);
    }
}

//DownAttackD
unsafe extern "C" fn effect_kamek_DownAttackD(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 5, 5, 12, -30, 0, 1, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
    }
    frame(fighter.lua_state_agent, 20.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_attack_arc"), false, false);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 3.8, -5, 0, 120, 23, 1, true, *EF_FLIP_YZ);
    }
}

//SlipAttack
unsafe extern "C" fn effect_kamek_SlipAttack(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 4.5, -6, 0, 120, 13, 0.9, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(fighter.lua_state_agent, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.1, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
        LAST_EFFECT_SET_ALPHA(fighter, 0.55);
    }
    frame(fighter.lua_state_agent, 28.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("rosetta_attack_arc"), Hash40::new("rosetta_attack_arc"), Hash40::new("top"), 0, 4.5, 5, 0, -50, -10, 1, true, *EF_FLIP_YZ);
    }
}

//SpecialNStart
unsafe extern "C" fn effect_kamek_SpecialNStart(fighter: &mut L2CAgentBase) {}

//SpecialNHold
unsafe extern "C" fn effect_kamek_SpecialNHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialNFire
unsafe extern "C" fn effect_kamek_SpecialNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//SpecialAirNStart
unsafe extern "C" fn effect_kamek_SpecialAirNStart(fighter: &mut L2CAgentBase) {}

//SpecialAirNHold
unsafe extern "C" fn effect_kamek_SpecialAirNHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 64.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("kamek_book"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true);
    }
}

//SpecialAirNFire
unsafe extern "C" fn effect_kamek_SpecialAirNFire(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        LAST_EFFECT_SET_COLOR(fighter, 255.0/255.0, 0.0/255.0, 60.0/255.0);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//SpecialS
unsafe extern "C" fn effect_kamek_SpecialS(fighter: &mut L2CAgentBase) {
    let magic_type = WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
    let rand_num = sv_math::rand(hash40("ness"), 100);
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("stg_mariou_water_magic_bright"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if magic_type == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_mariou_water_magic_bright"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//SpecialAirS
unsafe extern "C" fn effect_kamek_SpecialAirS(fighter: &mut L2CAgentBase) {
    let magic_type = WorkModule::get_int(fighter.module_accessor, FIGHTER_KAMEK_STATUS_SPECIAL_S_WORK_INT_MAGIC_TYPE);
    let rand_num = sv_math::rand(hash40("ness"), 100);
    frame(fighter.lua_state_agent, 22.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("stg_mariou_water_magic_bright"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.1, true);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 0.9, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(fighter.lua_state_agent, 34.0);
    if is_excute(fighter) {
        if magic_type == 1 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
        }
    }
    frame(fighter.lua_state_agent, 71.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_mariou_water_magic_bright"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
}

//SpecialHiStart
unsafe extern "C" fn effect_kamek_SpecialHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//SpecialAirHiStart
unsafe extern "C" fn effect_kamek_SpecialAirHiStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 15.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//SpecialHiEnd
unsafe extern "C" fn effect_kamek_SpecialHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_togezoshell_speedline"), false, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//SpecialAirHiEnd
unsafe extern "C" fn effect_kamek_SpecialAirHiEnd(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 2.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter,Hash40::new("sys_togezoshell_speedline"),Hash40::new("top"),0,-5,0,0,0,0,1.0,true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.85);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 0, 7.5, 1.225, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 12.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_togezoshell_speedline"), false, true);
    }
    frame(fighter.lua_state_agent, 13.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
    frame(fighter.lua_state_agent, 17.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 4.5, 0, 180, 90, 7.5, 1.125, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
        EFFECT_FOLLOW(fighter, Hash40::new("sys_spin_wind"), Hash40::new("top"), 0, 13.5, 0, 180, 0, 7.5, 0.8, true);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        LAST_EFFECT_SET_ALPHA(fighter, 0.5);
    }
}

//SpecialLwStart
unsafe extern "C" fn effect_kamek_SpecialLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 6, 2.5, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 22.0/255.0, 216.0/255.0, 0.0/255.0);
    }
    frame(fighter.lua_state_agent, 23.0);
    if is_excute(fighter) {
        FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
    }
}

//SpecialAirLwStart
unsafe extern "C" fn effect_kamek_SpecialAirLwStart(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, 2.5, 6, 0, 0, 0, 0.7, false);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        LAST_EFFECT_SET_COLOR(fighter, 22.0/255.0, 216.0/255.0, 0.0/255.0);
    }
}

//AppealLwR
unsafe extern "C" fn effect_kamek_AppealLwR(fighter: &mut L2CAgentBase) {}

//AppealLwL
unsafe extern "C" fn effect_kamek_AppealLwL(fighter: &mut L2CAgentBase) {}

//AppealSR
unsafe extern "C" fn effect_kamek_AppealSR(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

//AppealSL
unsafe extern "C" fn effect_kamek_AppealSL(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 17.0);
	if is_excute(fighter) {
		EFFECT(fighter, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 7, 8, 1, 0, 0, 0, 0.22, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_ALPHA(fighter, 0.8);
	}
}

//Final
unsafe extern "C" fn effect_kamek_Final(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        LAST_EFFECT_SET_COLOR(fighter, 111.0/255.0, 1.0, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
        LAST_EFFECT_SET_COLOR(fighter, 111.0/255.0, 1.0, 0.0);
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_mariou_water_magic_bright"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
    frame(fighter.lua_state_agent, 149.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
    }
}

//FinalAir
unsafe extern "C" fn effect_kamek_FinalAir(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_bg_black"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, true);
    }
    frame(fighter.lua_state_agent, 16.0);
    if is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 44.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_light"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.4, true);
        LAST_EFFECT_SET_COLOR(fighter, 111.0/255.0, 1.0, 0.0);
        EFFECT_FOLLOW(fighter, Hash40::new("rosetta_wand_stardust"), Hash40::new("throw"), 0, 0, 0, 0, 0, 0, 1.3, true);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
        LAST_EFFECT_SET_RATE(fighter, 2);
        LAST_EFFECT_SET_COLOR(fighter, 111.0/255.0, 1.0, 0.0);
        if PostureModule::lr(fighter.module_accessor) == 1.0 {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), -4, 6, 0, 0, 0, 0, 1.2, true);
        }
        else {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_flash"), Hash40::new("top"), 4, 6, 0, 0, 0, 0, 1.2, true);
        }
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 3.0, 1.0);
    }
    frame(fighter.lua_state_agent, 110.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("stg_mariou_water_magic_bright"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_light"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("rosetta_wand_stardust"), false, true);
    }
    frame(fighter.lua_state_agent, 149.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_bg_black"), false, false);
    }
}

pub fn install() {
    Agent::new("ness")
    .effect_acmd("effect_jumpaerialfront_kamek", effect_kamek_JumpAerialFront, Low)
    .effect_acmd("effect_jumpaerialback_kamek", effect_kamek_JumpAerialBack, Low)
    .effect_acmd("effect_entryl_kamek", effect_kamek_EntryL, Low)
    .effect_acmd("effect_entryr_kamek", effect_kamek_EntryR, Low)
    .effect_acmd("effect_attack11_kamek", effect_kamek_Attack11, Low)
    .effect_acmd("effect_attackdash_kamek", effect_kamek_AttackDash, Low)
    .effect_acmd("effect_attacks3hi_kamek", effect_kamek_AttackS3, Low)
    .effect_acmd("effect_attacks3_kamek", effect_kamek_AttackS3, Low)
    .effect_acmd("effect_attacks3lw_kamek", effect_kamek_AttackS3, Low)
    .effect_acmd("effect_attackhi3_kamek", effect_kamek_AttackHi3, Low)
    .effect_acmd("effect_attacklw3_kamek", effect_kamek_AttackLw3, Low)
    .effect_acmd("effect_attacks4_kamek", effect_kamek_AttackS4, Low)
    .effect_acmd("effect_attackhi4_kamek", effect_kamek_AttackHi4, Low)
    .effect_acmd("effect_attacklw4_kamek", effect_kamek_AttackLw4, Low)
    .effect_acmd("effect_attackairn_kamek", effect_kamek_AttackAirN, Low)
    .effect_acmd("effect_attackairf_kamek", effect_kamek_AttackAirF, Low)    
    .effect_acmd("effect_attackairb_kamek", effect_kamek_AttackAirB, Low)
    .effect_acmd("effect_attackairhi_kamek", effect_kamek_AttackAirHi, Low)
    .effect_acmd("effect_attackairlw_kamek", effect_kamek_AttackAirLw, Low)
    .effect_acmd("effect_catch_kamek", effect_kamek_Catch, Low)
    .effect_acmd("effect_catchdash_kamek", effect_kamek_CatchDash, Low)
    .effect_acmd("effect_catchturn_kamek", effect_kamek_CatchTurn, Low)
    .effect_acmd("effect_catchattack_kamek", effect_kamek_CatchAttack, Low)
    .effect_acmd("effect_catchwait_kamek", effect_kamek_CatchWait, Low)
    .effect_acmd("effect_catchpull_kamek", effect_kamek_CatchPull, Low)
    .effect_acmd("effect_throwf_kamek", effect_kamek_ThrowF, Low)
    .effect_acmd("effect_throwb_kamek", effect_kamek_ThrowB, Low)
    .effect_acmd("effect_throwhi_kamek", effect_kamek_ThrowHi, Low)
    .effect_acmd("effect_throwlw_kamek", effect_kamek_ThrowLw, Low)
    .effect_acmd("effect_downattackd_kamek", effect_kamek_DownAttackD, Low)
    .effect_acmd("effect_downattacku_kamek", effect_kamek_DownAttackU, Low)
    .effect_acmd("effect_cliffattack_kamek", effect_kamek_CliffAttack, Low)
    .effect_acmd("effect_slipattack_kamek", effect_kamek_SlipAttack, Low)
    .effect_acmd("effect_specialnstart_kamek", effect_kamek_SpecialNStart, Low)  
    .effect_acmd("effect_specialairnstart_kamek", effect_kamek_SpecialAirNStart, Low)
    .effect_acmd("effect_specialnhold_kamek", effect_kamek_SpecialNHold, Low)  
    .effect_acmd("effect_specialairnhold_kamek", effect_kamek_SpecialAirNHold, Low)  
    .effect_acmd("effect_specialnfire_kamek", effect_kamek_SpecialNFire, Low)  
    .effect_acmd("effect_specialairnfire_kamek", effect_kamek_SpecialAirNFire, Low)  
    .effect_acmd("effect_specials_kamek", effect_kamek_SpecialS, Low)  
    .effect_acmd("effect_specialairs_kamek", effect_kamek_SpecialAirS, Low) 
    .effect_acmd("effect_specialhistart_kamek", effect_kamek_SpecialHiStart, Low)
    .effect_acmd("effect_specialairhistart_kamek", effect_kamek_SpecialAirHiStart, Low)
    .effect_acmd("effect_speciallwstart_kamek", effect_kamek_SpecialLwStart, Low)
    .effect_acmd("effect_specialairlwstart_kamek", effect_kamek_SpecialAirLwStart, Low)
    .effect_acmd("effect_appealsr_kamek", effect_kamek_AppealSR, Low)
    .effect_acmd("effect_appealsl_kamek", effect_kamek_AppealSL, Low)
    .effect_acmd("effect_appeallwr_kamek", effect_kamek_AppealLwR, Low)
    .effect_acmd("effect_appeallwl_kamek", effect_kamek_AppealLwL, Low)
    .effect_acmd("effect_final_kamek", effect_kamek_Final, Low)
    .effect_acmd("effect_finalair_kamek", effect_kamek_FinalAir, Low)
    .install();
}