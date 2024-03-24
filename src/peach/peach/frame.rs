use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_peach_Exec(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
        let prev_status_0 = StatusModule::prev_status_kind(fighter.module_accessor, 0);
        let prev_status_1 = StatusModule::prev_status_kind(fighter.module_accessor, 1);
        if [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_0)
            || [*FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT, *FIGHTER_PEACH_STATUS_KIND_UNIQ_FLOAT_START].contains(&prev_status_1) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
        }
    }
    if [*FIGHTER_STATUS_KIND_ATTACK_S4_START,*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_S4].contains(&status_kind) {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("havel"), &Vector3f{ x: 2.5, y: 2.5, z: 2.5 });
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{ x: 2.0, y: 2.0, z: 2.0 });
        AttackModule::set_attack_scale(fighter.module_accessor, 2.5, true);
    }
    else {
        AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_PEACH_STATUS_KIND_SPECIAL_N_HIT].contains(&status_kind) {
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
    Agent::new("peach")
    .on_line(Exec, frame_peach_Exec)
    .install();
}