use crate::imports::BuildImports::*;

//SpecialSHold
unsafe extern "C" fn effect_luigi_SpecialSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
        WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_PULSE_EFFECT_HANDLE);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
        }
    }
    for _ in 0..100 {
        if is_excute(fighter) {
            FOOT_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
            let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
            WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_SMOKE_EFFECT_HANDLE);
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
                LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.7, 0.3);
            }
        }
        wait(fighter.lua_state_agent, 10.0);
    }
}

//SpecialAirSHold
unsafe extern "C" fn effect_luigi_SpecialAirSHold(fighter: &mut L2CAgentBase) {
    frame(fighter.lua_state_agent, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("luigi_rocket_hold"), Hash40::new("top"), 0, 6, 0, 0, 0, 0, 1, true);
        let handle = EffectModule::get_last_handle(fighter.module_accessor) as u32;
        WorkModule::set_int(fighter.module_accessor, handle as i32, *FIGHTER_LUIGI_INSTANCE_WORK_ID_INT_SPECIAL_S_PULSE_EFFECT_HANDLE);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
        }
    }
}

//SpecialSDischarge
unsafe extern "C" fn effect_luigi_SpecialSDischarge(fighter: &mut L2CAgentBase) {
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("luigi_rocket_bomb"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.8, 0.0);
    }
    frame(fighter.lua_state_agent, 3.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(fighter.lua_state_agent, 5.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("luigi_rocket_jet2"), Hash40::new("top"), 0, 4, 4, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.0, 0.7, 0.3);
        EffectModule::enable_sync_init_pos_last(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("luigi")
    .effect_acmd("effect_specialshold", effect_luigi_SpecialSHold, Low)
    .effect_acmd("effect_specialairshold", effect_luigi_SpecialAirSHold, Low)
    .effect_acmd("effect_specialsdischarge", effect_luigi_SpecialSDischarge, Low)
    .install();
}