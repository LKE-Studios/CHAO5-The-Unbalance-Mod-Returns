use crate::imports::BuildImports::*;
use crate::pit::param::SpecialHiFlyParams;

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_UNIQ, *GROUND_CORRECT_KIND_KEEP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = SpecialHiFlyParams::get();
    let frame = MotionModule::frame(fighter.module_accessor);
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, hash40("special_hi_start") as i64, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_MOTION_KIND);
        WorkModule::set_float(fighter.module_accessor, params.lr_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_LR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, params.dir_stick_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_STICK_X);
        WorkModule::set_float(fighter.module_accessor, params.dir_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_DIR_MUL);
        WorkModule::set_float(fighter.module_accessor, params.pass_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_PASS_MUL);
        WorkModule::set_float(fighter.module_accessor, params.accel_y, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_ACCEL_Y);
        WorkModule::set_float(fighter.module_accessor, params.start_x_mul, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_AIR_START_X_MUL);
        WorkModule::set_float(fighter.module_accessor, params.fall_x, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_FLOAT_CONST_FALL_X_MUL);
        WorkModule::set_int(fighter.module_accessor, FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
        fighter.super_jump_punch(L2CValue::Void());
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = SpecialHiFlyParams::get();
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    fighter.super_jump_punch_main();
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), true.into());
        if stick_x * lr < -0.25 {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), false.into());
        }
    }
    0.into()
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.super_jump_punch_end(L2CValue::Ptr(L2CFighterCommon_super_jump_punch_reset_common_condition as *const () as _));
	return 0.into();
}

//FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY
#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_FALL_FREE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = SpecialHiFlyParams::get();
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
    KineticUtility::reset_enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, *ENERGY_CONTROLLER_RESET_TYPE_FREE, Vector2f{x: 0.0, y: 0.0}, Vector3f{x: 0.0, y: 0.0, z: 0.0});
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL_FREE);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_fly_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_fly_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let params = SpecialHiFlyParams::get();
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ATTACK);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ITEM_THROW);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_TREAD_JUMP);
    if stick_x * lr < -0.25 {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), false.into());
    }
    if stick_y > 0.0 {
        MotionModule::set_rate(fighter.module_accessor, params.motion_rate_max);
    }
    else {
        MotionModule::set_rate(fighter.module_accessor, params.motion_rate_min);
    }
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if fighter.global_table[PREV_SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END.into(), false.into());
            WorkModule::set_float(fighter.module_accessor, landing_frame as f32, *FIGHTER_INSTANCE_WORK_ID_FLOAT_LANDING_FRAME);
            return 0.into();
        }
    }
    0.into()
}

#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn status_exec_pit_special_hi_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    pit_KineticTransactor_exec(fighter);
    0.into()
}

unsafe extern "C" fn pit_KineticTransactor_exec(fighter: &mut L2CFighterCommon) {
    let params = SpecialHiFlyParams::get();
    //let flick_x = ControlModule::get_flick_x(fighter.module_accessor);
    //let flick_y = ControlModule::get_flick_y(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor) * lr;
    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let mut speed_x = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_SPEED_X);
    let mut speed_y = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_SPEED_Y);
    if stick_x > 0.1 || stick_x < -0.1 {
        speed_x += 0.2 * stick_x;
        if speed_x > params.speed_x_max {
            speed_x = params.speed_x_max;
        }  
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: speed_x * params.air_accel_x, y: 0.0, z: 0.0});
    }
    if stick_y > 0.1 || stick_y < -0.1 {
        speed_y += 0.2 * stick_y;
        if speed_y > params.speed_y_max {
            speed_y = params.speed_y_max;
        }
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: 0.0, y: speed_y * params.air_accel_y, z: 0.0});
    }
    let mut new_gravity = params.gravity_speed + params.air_decel_y;
    if new_gravity > params.gravity_speed {
        new_gravity = params.gravity_speed;
    }
    speed_y -= new_gravity;
}

#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi_fly(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN
#[status_script(agent = "pit", status = 0x1ED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi_fly_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_MOTION_AIR, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, 0 as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = 0x1ED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi_fly_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_fly_turn_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_fly_turn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    pit_KineticTransactor_exec(fighter);
    fighter.sub_air_check_fall_common();
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), false.into());
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    0.into()
}

#[status_script(agent = "pit", status = 0x1ED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi_fly_turn(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_END
#[status_script(agent = "pit", status = 0x1EE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi_fly_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = 0x1EE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi_fly_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_end"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_fly_end_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_fly_end_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
    }
    0.into()
}

#[status_script(agent = "pit", status = 0x1EE, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi_fly_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_wait_ground_check_common(false.into());
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_pre_pit_special_hi,
        status_main_pit_special_hi,
        status_end_pit_special_hi,
        status_pre_pit_special_hi_fly,
        status_main_pit_special_hi_fly,
        status_exec_pit_special_hi_fly,
        status_end_pit_special_hi_fly,
        status_pre_pit_special_hi_fly_turn,
        status_main_pit_special_hi_fly_turn,
        status_end_pit_special_hi_fly_turn,
        status_pre_pit_special_hi_fly_end,
        status_main_pit_special_hi_fly_end,
        status_end_pit_special_hi_fly_end
    );
}