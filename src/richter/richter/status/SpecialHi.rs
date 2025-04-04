use crate::imports::BuildImports::*;

unsafe extern "C" fn status_richter_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
    WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let accel_y_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("accel_y_air"));
    let pass_mul_ground = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul_ground"));
    let pass_mul_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul_air"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
    WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
    WorkModule::set_float(fighter.module_accessor, accel_y_air, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
    WorkModule::set_float(fighter.module_accessor, pass_mul_ground, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, pass_mul_air, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
    WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
    WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_X_MUL);
    WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_MOVE_TRANS_END_SPEED_Y_MUL);
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI);
    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.super_jump_punch(L2CValue::Void());
    let start_x_mul;
    let start_y_mul;
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        let gravity = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        let limit_speed_y_air = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("limit_speed_y_air"));
        FighterKineticEnergyGravity::set_stable_speed(gravity as *mut smash::app::FighterKineticEnergyGravity, limit_speed_y_air);
        start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_x_mul_air"));
        start_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_y_mul_air"));
    }
    else {
        KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, Vector2f{x: sum_speed_x, y: 0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
        start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("start_x_mul_ground"));
        start_y_mul = 1.0;
    }
    KineticModule::mul_speed(fighter.module_accessor, &Vector3f{x: start_x_mul, y: start_y_mul, z: 0.0}, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    fighter.sub_shift_status_main(L2CValue::Ptr(richter_SpecialHi_Main_loop as *const () as _))
}

unsafe extern "C" fn richter_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_transition_group_check_air_cliff().get_bool() {
        return 0.into();
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    fighter.super_jump_punch_main();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE) {
        sv_kinetic_energy!(reset_energy, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, ENERGY_CONTROLLER_RESET_TYPE_FALL_ADJUST, 0.0, 0.0, 0.0, 0.0, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_STATUS_SPECIAL_HI_FLAG_MOVE);
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SIMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_HI);
        if frame > 32.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("richter")
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_richter_SpecialHi_Main)
    .install();
}