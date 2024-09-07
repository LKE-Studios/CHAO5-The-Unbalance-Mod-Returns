use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_ridley_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let mut power = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_POWER);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        let angle_se_pitch_partial_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_partial_ratio"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_glide_loop"), 0.8 + angle * angle_se_pitch_ratio);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_ridley_jump02"), 1.0 + angle * angle_se_pitch_partial_ratio);
        let angle_up_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_max"));
        let angle_down_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_max"));
        let angle_up_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_up_set_motion_rate_partial"));
        let angle_down_set_motion_rate_partial =  WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_down_set_motion_rate_partial"));
        if angle >= angle_down_max && angle < 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_down_set_motion_rate_partial);
        }
        if angle <= angle_up_max && angle > 0.0 {
            MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING, 1.0 + angle * angle_up_set_motion_rate_partial);
        }
        if MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) >= 25.0 
        && MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) < 26.0 {
            PLAY_SE(fighter, Hash40::new("se_ridley_jump02"));
        }
    }
    if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) { 
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_ridley_glide_loop"), 0);
    };
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_N_FAILURE, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FAILURE,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_JUMP, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_DRAG_WALL, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_S_FALL_JUMP,
        *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_END, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_CEIL, *FIGHTER_RIDLEY_STATUS_KIND_SPECIAL_HI_STOP_WALL].contains(&status_kind) {
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
    Agent::new("ridley")
    .on_line(Main, frame_ridley_Main)
    .install();
}