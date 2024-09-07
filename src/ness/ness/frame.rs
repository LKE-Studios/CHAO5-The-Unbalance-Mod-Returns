use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_ness_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD {
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        if stick_x * lr < 0.0 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            PostureModule::reverse_lr(fighter.module_accessor);
            PostureModule::update_rot_y_lr(fighter.module_accessor);
        }
    }
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if situation_kind == *SITUATION_KIND_AIR {
            if KineticModule::get_kinetic_type(fighter.module_accessor) != *FIGHTER_KINETIC_TYPE_FALL {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
            }
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_END,
    *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_FIRE, *FIGHTER_NESS_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_HI_END,
    *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD].contains(&status_kind) {
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
    Agent::new("ness")
    .on_line(Main, frame_ness_Main)
    .install();
}