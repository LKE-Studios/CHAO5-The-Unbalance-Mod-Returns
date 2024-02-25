use crate::imports::BuildImports::*;
use crate::metaknight::metaknight::status::SpecialHi::metaknight_SpecialHi_Main_loop;

unsafe extern "C" fn status_metaknight_SpecialHiLoop_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul"));
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y"));
    let air_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_x_mul"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let trans_move_end_speed_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_x_mul"));
    let trans_move_end_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("trans_move_end_speed_y_mul"));
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_loop") as i64,*FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, air_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, air_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, air_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, trans_move_end_speed_y_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_FLAG_SPECIAL_HI_GLIDE);
    fighter.super_jump_punch(L2CValue::Void());
    fighter.sub_shift_status_main(L2CValue::Ptr(metaknight_SpecialHi_Main_loop as *const () as _))
}

pub fn install() {
    Agent::new("metaknight")
    .status(Main, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP, status_metaknight_SpecialHiLoop_Main)
    .install();
}