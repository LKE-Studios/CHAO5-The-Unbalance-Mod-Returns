use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_krool_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_GET, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_CATCH,
    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_THROW, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_S_FAILURE, *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_FALL,
    *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI_AIR_END,].contains(&status_kind) {
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
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_HI {
        fighter.sub_air_check_fall_common();
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_AIR, false);
        }
    }
    if status_kind == *FIGHTER_KROOL_STATUS_KIND_SPECIAL_LW_HIT {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -40.0, 0);
        }
    };
}

pub fn install() {
    Agent::new("krool")
    .on_line(Main, frame_krool_Main)
    .install();
}