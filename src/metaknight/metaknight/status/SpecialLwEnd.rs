use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialLwEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    metaknight_special_lw_end_motion_handler(fighter);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialLwEnd_Main_Sub as *const () as _))
}

unsafe extern "C" fn metaknight_special_lw_end_motion_handler(fighter: &mut L2CFighterCommon) {
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_air_lw_end"), true, -1.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        let mot = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new("special_lw_end"), true, -1.0);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
}

unsafe extern "C" fn metaknight_SpecialLwEnd_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let mut bool_set: bool = false;
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
        if fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if !StatusModule::is_changing(fighter.module_accessor) {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
            if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
                bool_set = true;
            }
        }
    }
    if MotionModule::is_end(fighter.module_accessor) {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        }
    }
    if bool_set {
        metaknight_special_lw_end_motion_handler(fighter);
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END, status_metaknight_SpecialLwEnd_Main)
    .install();
}