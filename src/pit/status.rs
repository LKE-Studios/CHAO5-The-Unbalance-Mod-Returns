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
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        fighter.set_situation(SITUATION_KIND_AIR.into());
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_start"), 0.0, 1.0, false, 0.0, false, false);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR_TRANS));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING);
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY.into(), true.into());
    }
    0.into()
}

#[status_script(agent = "pit", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY
#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_ON_DROP_BOTH_SIDES), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | *FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON) as u64, *FIGHTER_STATUS_ATTR_START_TURN as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi_fly(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly"), 0.0, 1.0, true, 0.0, false, false);
    fighter.set_situation(SITUATION_KIND_AIR.into());
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
    /*let params = SpecialHiFlyParams::get();
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let stick_y = fighter.global_table[STICK_Y].get_f32();
    let stick_x = fighter.global_table[STICK_X].get_f32();
    let lr = PostureModule::lr(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    if motion_kind == hash40("special_hi_fly") {
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
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
        if stick_y > 0.0 {
            MotionModule::set_rate(fighter.module_accessor, params.motion_rate_min + (stick_y * params.motion_rate_mul));
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, params.motion_rate_min);
        }
        if lr > 0.0 && stick_x < 0.0 {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), false.into());
        }
        if lr < 0.0 && stick_x > 0.0 {
            fighter.change_status(FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN.into(), false.into());
        }
        let mut timer = WorkModule::get_int(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_INT_TIME);
        timer -= 1;
        let gravity = WorkModule::get_float(fighter.module_accessor, FIGHTER_PIT_STATUS_SPECIAL_HI_FLY_WORK_FLOAT_GRAVITY);
        let mut new_gravity = gravity + params.air_decel_y;
        if new_gravity > params.gravity_speed {
            new_gravity = params.gravity_speed;
        }
        let mut y_add;
        let mut x_add;
        x_add = (stick_x * lr) * params.air_accel_x;
        y_add = (stick_y) * params.air_accel_y;
        if x_add > 0.0 && LOG_X[ENTRY_ID] > params.speed_x_max {
            x_add = 0.0;
        };
        if x_add < 0.0 && LOG_X[ENTRY_ID] < params.speed_x_max * -1.0 {
            x_add = 0.0;
        };
        if y_add > 0.0 && LOG_Y[ENTRY_ID] > params.speed_y_max {
            y_add = 0.0;
        };
        if y_add < 0.0 && LOG_Y[ENTRY_ID] < params.speed_y_max * -1.0 {
            y_add = 0.0;
        };
        LOG_X[ENTRY_ID] += x_add;
        LOG_Y[ENTRY_ID] += y_add;
        if LOG_X[ENTRY_ID] > params.speed_x_max {
            LOG_X[ENTRY_ID] = params.speed_x_max;
        }
        if LOG_Y[ENTRY_ID] > params.speed_y_max {
            LOG_Y[ENTRY_ID] = params.speed_y_max;
        }
        LOG_Y[ENTRY_ID] -= new_gravity;
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{ x: x_add, y: y_add, z: 0.0 });
        if MotionModule::is_end(fighter.module_accessor) || timer == params.fly_end_frame {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), true.into());
        }
    }*/
    0.into()
}

#[status_script(agent = "pit", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_end_pit_special_hi_fly(_fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

//FIGHTER_PIT_STATUS_KIND_SPECIAL_HI_FLY_TURN
#[status_script(agent = "pit", status = 0x1ED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_pre_pit_special_hi_fly_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "pit", status = 0x1ED, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_main_pit_special_hi_fly_turn(fighter: &mut L2CFighterCommon) -> L2CValue {
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_hi_fly_turn"), 0.0, 1.0, false, 0.0, false, false);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    fighter.set_situation(SITUATION_KIND_AIR.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(pit_special_hi_fly_turn_loop as *const () as _))
}

unsafe extern "C" fn pit_special_hi_fly_turn_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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