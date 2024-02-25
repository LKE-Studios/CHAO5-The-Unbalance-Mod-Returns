use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_kamui_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW,
    *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_HOLD, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_BITE,
    *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_JUMP_END, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_JUMP, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK_END,
    *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_B, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_WALL_ATTACK_F, *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_S_ATTACK,
    *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_LW_HIT].contains(&status_kind) {
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
    let frame = MotionModule::frame(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
        if frame > 55.0 {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
}

pub fn install() {
    Agent::new("kamui")
    .on_line(Main, frame_kamui_Main)
    .install();
}