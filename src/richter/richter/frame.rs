use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_richter_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_y = if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
        ControlModule::get_sub_stick_y(fighter.module_accessor)
    }
    else {
        ControlModule::get_stick_y(fighter.module_accessor)
    };
    if [hash40("attack_air_f"), hash40("attack_air_f_hi"), hash40("attack_air_f_lw")].contains(&motion_kind) 
    && (11.0..12.0).contains(&frame) {
        if stick_y > 0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_f_hi"), -1.0, 1.0, 0.0, false, false);
        } else if stick_y < -0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_f_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else if [hash40("attack_air_b"), hash40("attack_air_b_hi"), hash40("attack_air_b_lw")].contains(&motion_kind) 
    && (11.0..12.0).contains(&frame) {
        if stick_y > 0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_b_hi"), -1.0, 1.0, 0.0, false, false);
        } else if stick_y < -0.5 {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("attack_air_b_lw"), -1.0, 1.0, 0.0, false, false);
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if situation_kind == *SITUATION_KIND_AIR {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
            }
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, false);
            }
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
        *FIGHTER_SIMON_STATUS_KIND_SPECIAL_S2].contains(&status_kind) {
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
    Agent::new("richter")
    .on_line(Main, frame_richter_Main)
    .install();
}