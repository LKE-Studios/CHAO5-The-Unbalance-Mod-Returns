use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_koopajr_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if status_kind == *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD {
        if frame > 16.0 {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                if situation_kind == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
                }
            }
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_DAMAGE_FLAG_END_REACTION)
    && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KOOPAJR_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_INTERRUPT) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_DAMAGE_END, false);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_HOLD,
    *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_JUMP, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_HIT_WALL,
    *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_S_SPIN_TURN, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_SHOOT, *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_FALL,
    *FIGHTER_KOOPAJR_STATUS_KIND_SPECIAL_HI_ATTACK].contains(&status_kind) {
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
    Agent::new("koopajr")
    .on_line(Main, frame_koopajr_Main)
    .install();
}