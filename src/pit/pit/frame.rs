use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_pit_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
    }
    if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_GLIDE].contains(&status_kind) { 
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_pit_glide_loop"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_pit_glide_start"), 0);
    };
    if status_kind == *FIGHTER_PIT_STATUS_KIND_SPECIAL_S_END || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -7.5, 0);
        }
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_PIT_STATUS_KIND_SPECIAL_N_SHOOT,
    *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_PIT_STATUS_KIND_SPECIAL_LW_END].contains(&status_kind) {
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
    if status_kind == *FIGHTER_STATUS_KIND_DEAD || !sv_information::is_ready_go() {
        let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
        WorkModule::set_float(fighter.module_accessor, fly_frame_max as f32, *FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER);
    } 
    else {
        let special_hi_fuel = WorkModule::get_float(fighter.module_accessor, *FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER);
        let recover_rate = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("recover_rate"));
        let fly_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fly_frame_max"));
        let new_fuel = (special_hi_fuel + recover_rate).min(fly_frame_max as f32);
        WorkModule::set_float(fighter.module_accessor, new_fuel, *FIGHTER_PIT_INSTANCE_WORK_ID_FLOAT_SPECIAL_HI_FLY_POWER);
    }
}

unsafe extern "C" fn frame_pit_Exec(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [
        *FIGHTER_STATUS_KIND_GLIDE_START,
        *FIGHTER_STATUS_KIND_GLIDE
    ].contains(&status_kind) { 
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.2, y:1.2, z:1.2});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.2, y:1.2, z:1.2});
    }
    else {
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingl1"), &Vector3f{x:1.0, y:1.0, z:1.0});
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("wingr1"), &Vector3f{x:1.0, y:1.0, z:1.0});
    };
}

pub fn install() {
    Agent::new("pit")
    .on_line(Main, frame_pit_Main)
    .on_line(Exec, frame_pit_Exec)
    .install();
}
