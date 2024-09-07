use crate::imports::BuildImports::*;

unsafe extern "C" fn status_lucario_SpecialHiBound_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_CLIFF);
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_bound"), 0.0, 1.0, false, 0.0, false, false);
    lucario_SpecialHiBound_ground_correct(fighter);
    if !GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        let speed_x = sv_kinetic_energy::get_speed_x(fighter.lua_state_agent);
        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        let crush_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_x_mul"));
        let crush_bound_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_bound_brake_x"));
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, crush_x_mul, 0.0);
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, crush_bound_brake_x, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    }
    else {
        let unknown_hash = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), 0x10f86b7c0c);
        let col_brake_deaccel = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("col_brake_deaccel"));
        let crush_bound_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_bound_speed_y"));
        let crush_bound_gravity = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("crush_bound_gravity"));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_BRAKE);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, unknown_hash);
        sv_kinetic_energy::mul_speed(fighter.lua_state_agent);
        sv_kinetic_energy!(set_brake, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, col_brake_deaccel, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        fighter.clear_lua_stack();
        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, crush_bound_speed_y); 
        sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -crush_bound_gravity);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(lucario_SpecialHiBound_Main_loop as *const () as _))
}

unsafe extern "C" fn lucario_SpecialHiBound_ground_correct(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR)); 
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND)); 
    }
}

unsafe extern "C" fn lucario_SpecialHiBound_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into()
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_AERIAL.into(), false.into());
        }
        if StatusModule::is_changing(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND && 
            fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            }
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_AIR && 
            fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            }
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
    }
    0.into()
}

pub fn install() {
    Agent::new("lucario")
    .status(Main, *FIGHTER_LUCARIO_STATUS_KIND_SPECIAL_HI_BOUND, status_lucario_SpecialHiBound_Main)
    .install();
}