use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialSEnd_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let end_rot_comp_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_s"), hash40("end_rot_comp_frame"));
    WorkModule::set_int(fighter.module_accessor, end_rot_comp_frame, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_INT_ROT_COMP_FRAME);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_s_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_end") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_finish") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_s_finish") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        let mot_air_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_air_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        }
        else {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
        }
        let mot_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(mot_kind), 0.0, 1.0, false, 0.0, false, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
        KineticModule::clear_speed_all(fighter.module_accessor);
        let end_air_speed_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_speed_x"));
        let end_air_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("end_air_speed_y"));
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: end_air_speed_x, y: end_air_speed_y, z: 0.0});
    }
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialSEnd_Main_Sub as *const () as _))
}

unsafe extern "C" fn metaknight_SpecialSEnd_Main_Sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        DamageModule::heal(fighter.module_accessor, -5.0, 0);
    }
    if situation_kind != *SITUATION_KIND_GROUND {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR);
        if MotionModule::is_end(fighter.module_accessor) {
            if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
                fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                return false.into();
            }
        }
    }
    else {
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
            let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
            if motion_kind != hash40("special_air_s_end") {
                if MotionModule::is_end(fighter.module_accessor) {
                    fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
                    return false.into();
                }
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_END_WORK_FLAG_SITUATION_AIR) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return false.into();
                }
            }
            else {
                if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_S_WORK_FLAG_HIT) {
                    fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
                    return false.into();
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_END, status_metaknight_SpecialSEnd_Main)
    .install();
}