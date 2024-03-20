use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_eflame(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_EFLAME_STATUS_KIND_SPECIAL_S_CATCH].contains(&status_kind) {
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
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) && 
        (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK)) { 
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
        }
    }
}

pub fn install() {
    Agent::new("eflame")
    .on_line(Main, frame_eflame)
    .install();
}