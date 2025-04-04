use crate::imports::BuildImports::*;

pub static lr_stick_x : f32 = 0.4;
pub static dir_stick_x : f32 = 0.6;
pub static dir_mul : f32 = 28.0;
pub static pass_mul : f32 = 1.6;
pub static air_accel_y : f32 = 0.4;
pub static air_start_x_mul : f32 = 1.0;
pub static air_pass_mul : f32 = 1.6;
pub static fall_x_mul : f32 = 1.0;
pub static landing_frame : i32 = 20;

unsafe extern "C" fn status_basyaamo_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, GROUND_CORRECT_KIND_KEEP.into(), GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
        FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, 0, FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI.into(), 0);
        return 0.into();
    }
    else {
        original_status(Pre, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn status_basyaamo_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
        WorkModule::set_int64(fighter.module_accessor, hash40("special_air_hi") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND_AIR);
        WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
        WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
        WorkModule::set_float(fighter.module_accessor, air_accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
        WorkModule::set_float(fighter.module_accessor, air_start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
        WorkModule::set_float(fighter.module_accessor, air_pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_PASS_MUL);
        WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
        WorkModule::set_int(fighter.module_accessor, landing_frame, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_CONST_LANDING_FRAME);
        WorkModule::set_float(fighter.module_accessor, 1.0, *FIGHTER_STATUS_WORK_ID_FLOAT_RESERVE_KINETIC_MOTION_SPEED_MUL);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.super_jump_punch(L2CValue::Void());
        WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
        fighter.sub_shift_status_main(L2CValue::Ptr(basyaamo_SpecialHi_Main_loop as *const () as _))
    }
    else {
        original_status(Main, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn basyaamo_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_main();
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 0.into();
        }
    }
    let frame = MotionModule::frame(fighter.module_accessor);
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        if frame > 32.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
    0.into()
}

unsafe extern "C" fn status_basyaamo_SpecialHi_Exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        return 0.into();
    }
    else {
        original_status(Exit, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

unsafe extern "C" fn status_basyaamo_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BASYAAMO = color >= 120 && color <= 127;
    if BASYAAMO {
        return 0.into();
    }
    else {
        original_status(End, fighter, *FIGHTER_STATUS_KIND_SPECIAL_HI)(fighter)
    }
}

pub fn install() {
    Agent::new("captain")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_basyaamo_SpecialHi_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_basyaamo_SpecialHi_Main)
    .status(Exit, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_basyaamo_SpecialHi_Exit)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_basyaamo_SpecialHi_End)
    .install();
}