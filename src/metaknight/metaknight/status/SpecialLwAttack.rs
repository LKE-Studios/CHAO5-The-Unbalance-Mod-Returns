use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialLwAttack_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    let mantle_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
    let mantle_mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_MOVE_DISTANCE);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_RIGHT_EDGE_DISTANCE_X);
    WorkModule::set_float(fighter.module_accessor, 0.0, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_FLOAT_LEFT_EDGE_DISTANCE_X);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let module_accessor = fighter.global_table[MODULE_ACCESSOR].get_i32();
    FighterSpecializer_Metaknight::check_edge_special_lw(fighter.module_accessor);
    if WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_air_neutral_x")) <= stick_x.abs() {
        if stick_x * lr > 0.0 {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_b") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_ottotto_lr(fighter.module_accessor, lr, 1.5) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
                }
            }
        }
        else {
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw_f") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if situation_kind == *SITUATION_KIND_GROUND {
                if GroundModule::is_ottotto_lr(fighter.module_accessor, lr * -1.0, 1.5) {
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
                }
            }
        }
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_lw") as i64, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
        if situation_kind == *SITUATION_KIND_GROUND {
            if GroundModule::is_ottotto(fighter.module_accessor, 1.5) {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_FLAG_STOP_X);
            }
        }
        fighter.set_situation(SITUATION_KIND_AIR.into());
    }
    let special_lw_situation_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    let special_lw_prev_situation_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND_PREV);
    WorkModule::set_int(fighter.module_accessor, situation_kind, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_SITUATION_KIND);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
    if special_lw_prev_situation_kind != *SITUATION_KIND_GROUND {
        if special_lw_prev_situation_kind != *SITUATION_KIND_AIR {
            if special_lw_situation_kind == *SITUATION_KIND_AIR {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);   
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_air_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
    }
    else {
        if special_lw_situation_kind != *SITUATION_KIND_AIR {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        }
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
        }
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);
        MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialLwAttack_Main_loop as *const () as _))
}

unsafe extern "C" fn metaknight_SpecialLwAttack_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            if fighter.sub_air_check_fall_common().get_bool() {
                return 1.into();
            }
        }
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_LW);
        if frame > 20.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 3.0);
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
            }
            else {
                fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
            }
        }
        else {
            let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
            let mantle_mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_KIND);
            let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
            let mantle_mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_ATTACK_WORK_INT_ARTICLE_MOT_AIR_KIND);
            if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
                if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
                    if MotionModule::motion_kind(fighter.module_accessor) == mot_kind {
                        return 0.into();
                    }
                }
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_air_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_air_kind), -1.0, 1.0, 0.0, false, false);
            }
            else {
                GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT) {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_LW_FLAG_CONTINUE_MOT);
                }
                ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_METAKNIGHT_GENERATE_ARTICLE_MANTLE, Hash40::new_raw(mantle_mot_kind), true, -1.0);          
                MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(mot_kind), -1.0, 1.0, 0.0, false, false);
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK, status_metaknight_SpecialLwAttack_Main)
    .install();
}