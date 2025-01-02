use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_mario_Main(fighter: &mut L2CFighterCommon) {
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let mut on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL) {
        MotionModule::set_rate(fighter.module_accessor, 0.4);
        if frame > 14.0 {
            MotionModule::set_rate(fighter.module_accessor, 1.0); 
        } 
        if frame > 16.0 {
            MotionModule::set_rate(fighter.module_accessor, 0.6);
            if situation_kind == *SITUATION_KIND_AIR {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI);
                WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_JUMP_AERIAL);
            }
        } 
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME);
        }
        if on_frame >= 30 {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL);
        }
    }
    else {
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_MARIO_INSTANCE_WORK_ID_INT_SPECIAL_BUTTON_ON_FRAME)
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status_kind) {
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
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) == 9 {//Ice Mario Costume c09
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARIO_STATUS_SPECIAL_N_FLAG_SPECIAL_N_GIANT_FIREBALL);
    };
}

pub fn install() {
    Agent::new("mario")
    .on_line(Main, frame_mario_Main)
    .install();
}