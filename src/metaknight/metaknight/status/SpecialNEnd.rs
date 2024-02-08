use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialNEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_n_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_n_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_TRANSITION_TERM_ID_MOT_END);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_FRAME);
    let remove_effect_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("remove_effect_frame"));
    WorkModule::set_int(fighter.module_accessor, remove_effect_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE);
    metaknight_special_n_end_motion_handler(fighter);
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialNEnd_Main_Sub as *const () as _))
}

unsafe extern "C" fn metaknight_special_n_end_motion_handler(fighter: &mut L2CFighterCommon) {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let sum_speed = KineticModule::get_sum_speed(fighter.module_accessor, -1);
    fighter.Vector2__create(sum_speed.into(), sum_speed.into());
    if situation_kind != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        fighter.set_situation(SITUATION_KIND_AIR.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_end"), 0.0, 1.0, false, 0.0, false, false);
        let air_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_speed_x_mul"));
        let air_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_brake_x"));
        let air_end_max_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_end_max_speed_x"));
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, get_sum_speed_x * air_end_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_end_max_speed_x, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, air_end_brake_x, 0.0);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_end"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        let ground_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("ground_end_speed_x_mul"));
        let ground_end_brake_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("ground_end_brake_x"));
        let get_sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, get_sum_speed_x * ground_end_speed_x_mul, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, -1.0, 0.0);
        sv_kinetic_energy!(set_brake, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, ground_end_brake_x, 0.0);
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_FLAG_MOTION_TRANSITION_TERM_ID_MOT_END);
    WorkModule::set_int(fighter.module_accessor, *SITUATION_KIND_AIR, *FIGHTER_STATUS_WORK_ID_UTILITY_WORK_INT_MTRANS);
}

unsafe extern "C" fn metaknight_SpecialNEnd_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_end"), -1.0, 1.0, 0.0, false, false);
        }
    }
    else {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_end"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            fighter.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        }
    }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE) {
        let remove_effect_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("remove_effect_frame"));
        let end_frame = MotionModule::end_frame(fighter.module_accessor) as i32;
        if end_frame <= remove_effect_frame {
            WorkModule::set_int(fighter.module_accessor, end_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
        }
    }
    else {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_FLAG_EFFECT_REMOVE);
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    let spin_effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
    let spin_effect_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_FRAME);
    let spin_effect_remove_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_REMOVE_FRAME);
    if spin_effect_handle != 0 {
        if spin_effect_remove_frame <= spin_effect_frame {
            EffectModule::detach(fighter.module_accessor, 5.0 as u32, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_N_SPIN_WORK_INT_EFFECT_HANDLE);
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        else {
            return 0.into();
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
        else {
            return 0.into();
        }
    }
    let mtrans = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MTRANS);
    let mut bool_set: bool = false;
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != mtrans {
            if fighter.global_table[SITUATION_KIND].get_i32() == mtrans {
                bool_set = true;
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_TRANSITION_TERM_ID_MOT_END) {
            if MotionModule::is_end(fighter.module_accessor) {
                bool_set = true;
            }
        }
    }
    if bool_set {
        metaknight_special_n_end_motion_handler(fighter);
        if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_END, status_metaknight_SpecialNEnd_Main)
    .install();
}