use crate::imports::BuildImports::*;

unsafe extern "C" fn status_metaknight_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul"));
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y"));
    let air_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_x_mul"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let trans_move_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_x_mul"));
    let trans_move_end_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_y_mul"));
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, air_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_y_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi_start") as i64, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        let metaknight_aerial_motion_kind = WorkModule::get_int64(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_WORK_INT_MOT_AIR_KIND);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT) {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new_raw(metaknight_aerial_motion_kind), -1.0, 1.0, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new_raw(metaknight_aerial_motion_kind), 0.0, 1.0, false, 0.0, false, false);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_STATUS_SPECIAL_HI_FLAG_CONTINUE_MOT);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialHi_Main_loop_Sec as *const () as _))
    }
    else {
        fighter.super_jump_punch(L2CValue::Void());
        fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialHi_Main_loop as *const () as _))
    }
}

pub unsafe extern "C" fn metaknight_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if frame > 23.0 {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), true.into());
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            let super_jump_punch_end_status = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
            fighter.change_status(super_jump_punch_end_status.into(), false.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn metaknight_SpecialHi_Main_loop_Sec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    fighter.super_jump_punch_main();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if frame > 23.0 {
        WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
        if situation_kind == *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_LANDING_LIGHT.into(), true.into());
        }
    }
    if !fighter.sub_transition_group_check_air_cliff().get_bool() {
        if MotionModule::is_end(fighter.module_accessor) {
            fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP.into(), true.into());
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_metaknight_SpecialHi_Main)
    .install();
}