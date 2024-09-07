use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_lucario_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_HOLD,
    *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_CANCEL, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_N_MAX,
    *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
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
    if status_kind == FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_RUSH_END {
        fighter.sub_air_check_fall_common();
        if MotionModule::frame(fighter.module_accessor) < 1.0 {
            MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, 0.0, true, true, false);
        }
    }
}

pub fn install() {
    Agent::new("lucario")
    .on_line(Main, frame_lucario_Main)
    .install();
}