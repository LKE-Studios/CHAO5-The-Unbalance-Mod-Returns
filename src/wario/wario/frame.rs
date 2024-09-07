use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_wario_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_END,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_OPEN_WAIT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_START, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE_END,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_EAT, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BITE, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_BOMB,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_FOOD, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_ITEM, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_N_LARGE,
        *FIGHTER_WARIO_STATUS_KIND_SPECIAL_S_BUMP, *FIGHTER_WARIO_STATUS_KIND_SPECIAL_HI_JUMP].contains(&status_kind) {
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
    if status_kind == *FIGHTER_STATUS_KIND_THROW && motion_kind == hash40("throw_b") {
        if situation_kind == *SITUATION_KIND_GROUND {
            let maxFrame = 46.0;
            if stick_x != 0.0 && frame < maxFrame {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f{x: 1.0 * stick_x.signum(), y: 0.0, z: 0.0});
            }
        }
    }
}

pub fn install() {
    Agent::new("wario")
    .on_line(Main, frame_wario_Main)
    .install();
}