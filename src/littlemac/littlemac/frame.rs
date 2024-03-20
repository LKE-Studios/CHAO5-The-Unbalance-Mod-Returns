use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_littlemac_Main(fighter : &mut L2CFighterCommon) {       
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_S);
    if [*FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_BLOW_END,
    *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_S_JUMP_END, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {
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
    Agent::new("littlemac")
    .on_line(Main, frame_littlemac_Main)
    .install();
}