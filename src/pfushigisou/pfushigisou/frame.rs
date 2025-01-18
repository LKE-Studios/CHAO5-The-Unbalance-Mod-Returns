use crate::imports::BuildImports::*;
use crate::pfushigisou::pfushigisou::status::SpecialGuard::*;

unsafe extern "C" fn frame_pfushigisou_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), false.into());
    }
    let int_charge = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_CHARGE);
    let charge_effect_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_min"));
    let charge_effect_scale_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_max"));
    let effect_counter = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    let mut size_mul = int_charge as f32 * 0.005;
    if int_charge > 0 {
        WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    }
    size_mul = size_mul.clamp(charge_effect_scale_min, charge_effect_scale_max);
    if effect_counter >= 8 {
        let effect_a = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, size_mul * 0.6, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(fighter.module_accessor, effect_a as u32, 2.0, 1.4, 0.0);
        let effect_b = EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, size_mul, true, 0, 0, 0, 0, 0, true, true);
        EffectModule::set_rgb(fighter.module_accessor, effect_b as u32, 1.1, 1.4, 0.0);
        WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    }
    println!("{}", size_mul);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_END, 
        *FIGHTER_STATUS_KIND_SPECIAL_GUARD, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT].contains(&status_kind) {
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
    Agent::new("pfushigisou")
    .on_line(Main, frame_pfushigisou_Main)
    .install();
}