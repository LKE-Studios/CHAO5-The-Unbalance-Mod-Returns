use crate::imports::BuildImports::*;
use crate::pfushigisou::pfushigisou::status::SpecialGuard::*;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut EFFECT_SCALE_MUL : [f32; 8] = [0.1; 8];

unsafe extern "C" fn frame_pfushigisou_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, false);
    }
    let mut charge = WorkModule::get_int(fighter.module_accessor, FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_GUARD_INT_CHARGE);
    let charge_effect_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_min"));
    let charge_effect_scale_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_max"));
    if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] >= 1 {
        GFX_COUNTER[ENTRY_ID] += 1;
        if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD)) || status_kind == FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD {
            EFFECT_SCALE_MUL[ENTRY_ID] += 0.006;
        }
        else {
            EFFECT_SCALE_MUL[ENTRY_ID] += 0.0;
        }
        EFFECT_SCALE_MUL[ENTRY_ID] = EFFECT_SCALE_MUL[ENTRY_ID].clamp(charge_effect_scale_min, charge_effect_scale_max);
        if GFX_COUNTER[ENTRY_ID] >= 8 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_elec"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, EFFECT_SCALE_MUL[ENTRY_ID] * 0.6, true, 0, 0, 0, 0, 0, true, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 2.0, /*G*/ 1.4, /*B*/ 0.0);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_damage_fire"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, EFFECT_SCALE_MUL[ENTRY_ID], true, 0, 0, 0, 0, 0, true, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.1, /*G*/ 1.0, /*B*/ 0.0);
            GFX_COUNTER[ENTRY_ID] = 0;
        }
    } 
    else {
        EFFECT_SCALE_MUL[ENTRY_ID] = charge_effect_scale_min;
    }
    if !sv_information::is_ready_go() {
        PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] = 0;
        EFFECT_SCALE_MUL[ENTRY_ID] = charge_effect_scale_min;
    }
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N_END, 
    FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD_SHOOT].contains(&status_kind) {
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