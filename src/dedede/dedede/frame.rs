use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_dedede_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("jump_aerial_f9") {
        if MotionModule::frame(fighter.module_accessor) == 55.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
    }
    if status_kind == *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE && situation_kind == SITUATION_KIND_AIR {
        StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_END,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SPIT, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_SWALLOW,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_FALL, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP1, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_JUMP2,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_TURN_AIR, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_FALL, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_WAIT_JUMP,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_N_EAT_PASS, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_S_MISS,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_FAILURE, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_HI_TURN, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_JUMP,
    *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_FALL, *FIGHTER_DEDEDE_STATUS_KIND_SPECIAL_LW_PASS].contains(&status_kind) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH);
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
}

pub fn install() {
    Agent::new("dedede")
    .on_line(Main, frame_dedede_Main)
    .install();
}