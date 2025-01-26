use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_pzenigame_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), true.into());
    }
    if status_kind == *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT {
        HitModule::set_whole(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_HIT,
        *FIGHTER_PZENIGAME_STATUS_KIND_SPECIAL_S_END, *FIGHTER_STATUS_KIND_SPECIAL_GUARD].contains(&status_kind) {
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
    Agent::new("pzenigame")
    .on_line(Main, frame_pzenigame_Main)
    .install();
}