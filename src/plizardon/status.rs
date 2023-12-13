use crate::imports::BuildImports::*;

static AIR_SPEED_X : f32 = 0.4;
static DIVE_SPEED_Y : f32 = 4.4;

#[status_script(agent = "plizardon", status = FIGHTER_STATUS_KIND_SPECIAL_HI, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_plizardon_special_hi_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("lr_stick_x"));
    let dir_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_stick_x"));
    let dir_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("dir_mul"));
    let pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("pass_mul"));
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_accel_y"));
    let air_start_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_start_x_mul"));
    let air_pass_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("air_pass_mul"));
    let fall_x_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_hi"), hash40("fall_x_mul"));
    let landing_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_hi"), hash40("landing_frame"));
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
    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_WORK_INT_STATUS_KIND_END);
    fighter.super_jump_punch(L2CValue::Void());
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);
    fighter.sub_shift_status_main(L2CValue::Ptr(plizardon_special_hi_main_loop as *const () as _))
}

unsafe extern "C" fn plizardon_special_hi_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = MotionModule::frame(fighter.module_accessor);
    fighter.super_jump_punch_main();
    if frame >= 50.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2.into(), true.into());
        }
    }
    0.into()
}

//FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2
#[status_script(agent = "plizardon", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_plizardon_special_hi2_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_RESET, *GROUND_CORRECT_KIND_AIR as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "plizardon", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_plizardon_special_hi2_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_AIR.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2_start"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    fighter.sub_shift_status_main(L2CValue::Ptr(plizardon_special_hi2_loop as *const () as _))
}

unsafe extern "C" fn plizardon_special_hi2_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
        return 1.into();
    }
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if fighter.global_table[SITUATION_KIND].get_i32() != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP);
        sv_kinetic_energy!(clear_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    } 
    else {
        fighter.set_situation(SITUATION_KIND_GROUND.into());
        fighter.change_status(FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2_LANDING.into(), false.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2"), 0.0, 1.0, true, 0.0, false, true);
    }
    0.into()
}

#[status_script(agent = "plizardon", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe extern "C" fn status_plizardon_special_hi2_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::motion_kind(fighter.module_accessor) == hash40("special_air_hi_2") {
        fighter.sub_fighter_cliff_check(GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES.into());
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
        KineticModule::add_speed(fighter.module_accessor, &Vector3f{x: AIR_SPEED_X, y: -DIVE_SPEED_Y, z: 0.0});
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        }
    }
    0.into()
}

#[status_script(agent = "plizardon", status = 0x1DB, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_plizardon_special_hi2_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let situation_kind = fighter.global_table[SITUATION_KIND].get_i32();
    if situation_kind != *SITUATION_KIND_GROUND {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    else {
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    }
    0.into()
}

//FIGHTER_PLIZARDON_STATUS_KIND_SPECIAL_HI2_LANDING
#[status_script(agent = "plizardon", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn status_plizardon_special_hi2_landing_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_NONE), *FIGHTER_KINETIC_TYPE_NONE, *GROUND_CORRECT_KIND_GROUND as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_NO_REAC, false, false, false, (*FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_HI | *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK) as u64, *FIGHTER_STATUS_ATTR_CLEAR_MOTION_ENERGY as u32, *FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_HI as u32, 0);
    0.into()
}

#[status_script(agent = "plizardon", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn status_plizardon_special_hi2_landing_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.set_situation(SITUATION_KIND_GROUND.into());
    MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_hi_2_landing"), 0.0, 1.0, false, 0.0, false, false);
    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
    GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    fighter.sub_shift_status_main(L2CValue::Ptr(plizardon_special_hi2_landing_loop as *const () as _))
}

unsafe extern "C" fn plizardon_special_hi2_landing_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
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

#[status_script(agent = "plizardon", status = 0x1D3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn status_plizardon_special_hi2_landing_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_plizardon_special_hi_main,
        status_plizardon_special_hi2_pre,
        status_plizardon_special_hi2_main,
        status_plizardon_special_hi2_exec,
        status_plizardon_special_hi2_end,
        status_plizardon_special_hi2_landing_pre,
        status_plizardon_special_hi2_landing_main,
        status_plizardon_special_hi2_landing_end
    );
}