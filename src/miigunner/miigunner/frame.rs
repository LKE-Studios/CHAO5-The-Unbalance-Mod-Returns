use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_miigunner_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let CUSTOMIZE_TO = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_WAZA_CUSTOMIZE_TO);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_1,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_1, 
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_1].contains(&CUSTOMIZE_TO) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_FIRE, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_HOLD, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_START,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_END, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_HIT,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW1_LOOP].contains(&status_kind) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_2,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_2].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S2_END, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI2_JUMP].contains(&status_kind)) 
    || ([*FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_N_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_S_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_HI_3,
        *FIGHTER_WAZA_CUSTOMIZE_TO_SPECIAL_LW_3].contains(&CUSTOMIZE_TO)) 
    && [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_LOOP, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N3_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_1_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_S3_2_AIR, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_HI3_RUSH_END,
        *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_END, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HIT, *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_LW3_HOLD].contains(&status_kind) {
        if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
            fighter.sub_air_check_dive();
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
                    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                }
            }
        }
    }
    if status_kind == *FIGHTER_MIIGUNNER_STATUS_KIND_SPECIAL_N1_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(fighter.module_accessor, *STATUS_KIND_NONE, *FIGHTER_MIIGUNNER_STATUS_GUNNER_CHARGE_WORK_INT_CANCEL_STATUS);
            }
        }
    };
}

pub fn install() {
    Agent::new("miigunner")
    .on_line(Main, frame_miigunner_Main)
    .install();
}