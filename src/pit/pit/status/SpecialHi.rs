use crate::imports::BuildImports::*;

pub unsafe extern "C" fn status_pit_SpecialHi_Pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHi_Main(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_PIT_STATUS_SPECIAL_HI_FLAG_CONTINUE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("lr_stick_x"));
        let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("dir_mul"));
        let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("dir_stick_x"));
        let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("pass_mul"));
        let accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_start_x_mul"));
        let start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("air_accel_y"));
        let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi_fly"), hash40("fall_x_mul"));
        WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_start") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
        WorkModule::set_float(fighter.module_accessor, lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
        WorkModule::set_float(fighter.module_accessor, pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
        WorkModule::set_float(fighter.module_accessor, accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
        WorkModule::set_float(fighter.module_accessor, start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
        WorkModule::set_float(fighter.module_accessor, fall_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
        WorkModule::set_int(fighter.module_accessor, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.super_jump_punch(L2CValue::Void());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_SpecialHi_Main_loop as *const () as _))
}

unsafe extern "C" fn pit_SpecialHi_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_main();
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), true.into());
        let lr = PostureModule::lr(fighter.module_accessor);
        let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
        if stick_x * lr < -0.25 {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), true.into());
        }
    }
    0.into()
}

pub unsafe extern "C" fn status_pit_SpecialHi_End(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_end(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_reset_common_condition as *const () as _));
	return 0.into();
}

pub fn install() {
    Agent::new("pit")
    .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_pit_SpecialHi_Pre)
    .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_pit_SpecialHi_Main)
    .status(End, *FIGHTER_STATUS_KIND_SPECIAL_HI, status_pit_SpecialHi_End)
    .install();
}