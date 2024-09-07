use crate::imports::BuildImports::*;

pub static special_hi_speed_x_min : f32 = 0.6;
pub static special_hi_accel_x_add : f32 = 0.03;
pub static special_hi_speed_x_max : f32 = 1.4;
pub static special_hi_speed_y_max : f32 = 1.9;
pub static special_hi_speed_y_min : f32 = 0.75; 

unsafe extern "C" fn status_silver_SpecialHi3_Init(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        let sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FREE);
        sv_math::vec2_normalize(sum_speed_x, sum_speed_y);
        KineticModule::clear_speed_all(fighter.module_accessor);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, special_hi_speed_y_max);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, special_hi_speed_y_max);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, special_hi_speed_x_max, 0.0);
        sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, special_hi_speed_x_max, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            let wrap_xy_speed_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_xy_speed_air"));
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, wrap_xy_speed_air, wrap_xy_speed_air);
        }
        else {
            let wrap_xy_speed = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("wrap_xy_speed"));
            sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, wrap_xy_speed, 0.0);
        }
        VisibilityModule::set_whole(fighter.module_accessor, true);
        0.into()
    }
    else {
        original_status(Init, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3)(fighter)
    }
}

unsafe extern "C" fn status_silver_SpecialHi3_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SILVER = color >= 120 && color <= 127;
    if SILVER {
        let landing_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
        let fall_x_mull_value = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mull_value"));
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi"), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_GROUND, *FIGHTER_MEWTWO_STATUS_SPECIAL_HI_WORK_INT_START_SITUATION);
        }
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FREE, 0.0, 0.0, 0.0, 0.0, 0.0);
        WorkModule::set_float(fighter.module_accessor, landing_frame, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
        WorkModule::set_float(fighter.module_accessor, fall_x_mull_value, *FIGHTER_INSTANCE_WORK_ID_FLOAT_FALL_X_MAX_MUL);
        fighter.sub_shift_status_main(L2CValue::Ptr(silver_SpecialHi3_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3)(fighter)
    }
}

unsafe extern "C" fn silver_SpecialHi3_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let speed_x = special_hi_accel_x_add * stick_x;
    sv_kinetic_energy!(set_accel, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_x * lr, 0.0);
    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, 0.0, special_hi_speed_y_max);
    if MotionModule::is_end(fighter.module_accessor) {
        if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
            return 0.into();
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            return 0.into();
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("mewtwo")
    .status(Init, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, status_silver_SpecialHi3_Init)
    .status(Main, *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_HI_3, status_silver_SpecialHi3_Main)
    .install();
}
